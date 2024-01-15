# Adding a class

> ⚠️Warning⚠️  
> As the project evolves, some of the information in this document may become outdated, please use other already existing classes as a reference.

Let's say we want to add the [`TH1D`](https://root.cern/doc/v630/classTH1D.html) class to our project,
 which is a 1D histogram with double precision bins.

## Adding the C bindings

<!-- TODO add link -->
Create a new file `root-rs-c-bindings/include/root-rs-c-bindings/TH1D.h` and `root-rs-c-bindings/src/TH1D.cpp`. Add the following content to `root-rs-c-bindings/include/root-rs-c-bindings/_all.h`:
```c
// ...
#include "./TH1D.h"
```

Save the `CMakeLists.txt` file to make sure VSCode reloads it, otherwise we will not have code completion for the new files.

We now open the [`TH1D` documentation](https://root.cern/doc/v630/classTH1D.html) and we find the following inheritance diagram:

![TH1D inheritance](https://root.cern/doc/v630/classTH1D__inherit__graph.svg)

We observe that `TH1D` inherits from `TH1` and `TArrayD`. We then open `root-rs-c-bindings/include/root-rs-c-bindings/TH1D.h` and add the following content (if you are using VSCode, you can use the `root-obj-header` snippet):
```cpp
RRS_CLASS(TH1D);
RRS_CLASS_PARENT(TH1D, TH1);
RRS_CLASS_PARENT(TH1D, TArrayD);

#ifdef ROOT_RS_INCLUDE_INCLUDES
#include <TH1D.h>
#endif

#ifdef ROOT_RS_INCLUDE_FUNCTIONS
//void RRS_METHOD(TH1D, some_method)(some_args);
#endif
```

We then open `root-rs-c-bindings/src/TH1D.cpp` and add the following content (`root-obj-impl` snippet):
```cpp
#include <root-rs-c-bindings.h>

#define ROOT_RS_INCLUDE_INCLUDES
#include <root-rs-c-bindings/TH1D.h>
#undef ROOT_RS_INCLUDE_INCLUDES

extern "C" {
}
```

Again, we save the `CMakeLists.txt` file to make sure VSCode reloads it.

## Updating the Rust side

We add a new file named, for example, `src/histogram/th1d.rs` and declare it in the parent module.

We then add the following content to `src/histogram/th1d.rs`:
```rust
use crate::impl_utils::*;

root_object! {
    /// Some documentation
    TH1D
}
```
Now, since `TH1D` inherits from `TH1` and `TArrayD`, we modify it into:
```rust
root_object! {
    TH1D(TH1, TArrayD)
}
```
This provides some casting methods that will allow the user to up-cast the object to its parent classes. However, if the user wants to upcast from, tet's say, `TH1D` to `TObject`, he will have to upcast to `TH1` first and then to `TNamed` and finally to `TObject`. This is not ideal. To solve this, we add the following:
```rust
root_object! {
    TH1D(TH1, TArrayD),
    ref shortcuts:
    TNamed: TH1,
    TAttLine: TH1,
    TAttFill: TH1,
    TAttMarker: TH1,
    TObject: TH1 => TNamed,
    TArray: TArrayD,
}
```
This will add some shortcuts to up-cast the object to its parent classes. We have to do it manually since C++ supports _diamond inheritance_ and multiple paths might lead to the same parent class. This would make it difficult (and in some cases ambiguous) to automatically generate the code.

In Rust, coercion is implemented by the `Deref` trait. This means we can do coercion for only one class at a time.  
For `TH1D`, the user might often want to access the `TH1` methods, so we mark `TH1` as the default coercion target using a `ref`:
```rust
root_object! {
    TH1D(ref TH1, TArrayD),
    ref shortcuts:
    TNamed: TH1,
    TAttLine: TH1,
    TAttFill: TH1,
    TAttMarker: TH1,
    TObject: TH1 => TNamed,
    TArray: TArrayD,
}
```

## Adding methods

As an example, we want to add a constructor for the `TH1D` class.

We update the `root-rs-c-bindings/include/root-rs-c-bindings/TH1D.h` file:
```cpp
RRS_CLASS(TH1D);
RRS_CLASS_PARENT(TH1D, TH1);
RRS_CLASS_PARENT(TH1D, TArrayD);

#ifdef ROOT_RS_INCLUDE_INCLUDES
#include <TH1D.h>
#endif

#ifdef ROOT_RS_INCLUDE_FUNCTIONS
RRS_STRUCT(TH1D) RRS_METHOD(TH1D, new_range)(
    const char* RRS_REF name,
    const char* RRS_REF title,
    int n_bins_x,
    double x_low,
    double x_up
);
#endif
```

We then update the `root-rs-c-bindings/src/TH1D.cpp` file:
```cpp
#include <root-rs-c-bindings.h>

#define ROOT_RS_INCLUDE_INCLUDES
#include <root-rs-c-bindings/TH1D.h>
#undef ROOT_RS_INCLUDE_INCLUDES

extern "C" {
    RRS_STRUCT(TH1D) RRS_METHOD(TH1D, new_range)(
        const char* RRS_REF name,
        const char* RRS_REF title,
        int n_bins_x,
        double x_low,
        double x_up
    ) {
        return new TH1D(name, title, n_bins_x, x_low, x_up);
    }
}
```

We then update the `src/histogram/th1d.rs` file:
```rust
use std::ops::RangeInclusive;

use crate::impl_utils::*;

root_object! {
    TH1D(TH1, TArrayD),
    ref shortcuts:
    TNamed: TH1,
    TAttLine: TH1,
    TAttFill: TH1,
    TAttMarker: TH1,
    TObject: TH1 => TNamed,
    TArray: TArrayD,
}

impl TH1D {
    pub fn new_range(name: &str, title: &str, n_bins: usize, range: RangeInclusive<f64>) -> Ptr<Self> {
        to_c_str!(name, title);
        unsafe {
            let ptr = ffi_method!(TH1D::new_range)(
                name,
                title,
                n_bins as i32,
                *range.start() as f64,
                *range.end() as f64,
            );
            Ptr::new(ptr).expect("TH1F::new_range failed")
        }
    }
}
```