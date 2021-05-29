# Homogenous Collections

Whilst it is a nice feature to be able to store multiple different types in the same collection there are some distinct downsides.

- Virtual method calls must be performed through dynamic dispatch since the type of the data is unknown until runtime.
- Additional space must be used to store the addresses of the virtual methods on each member of the collection.
- Different types may have different sizes. Only references can be stored in the collection, not the data itself.

This is achieved by storing a reference to a vtable as part of the structure. The vtable contains the addresses of any virtual methods which can be retrieved and called dynamically.

We can save both time and space when it is known ahead of time that all of the data in the collection has the same type i.e. the collection is homogenous:

- Instead of storing one vtable pointer for every member of the collection, we only need one vtable pointer for the entire collection. Therefore, the vtable no longer needs to be stored alongside the data itself.
- If the type is known at compile time, then the vtable can be resolved before the program is run. This means that static dispatch can be used instead of dynamic dispatch.
- The size of the members can be known ahead of time, and can be stored directly inside the structure without using any indirection.