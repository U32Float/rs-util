#[macro_export]
/// Identical to `tracing::warn!`, but ensures that the warning is logged only once per unique identifier.
/// If no id is provided, it uses the call site as the unique identifier.
///
/// # Example:
/// ```rust
/// use rs_util::warn_once;
///
/// warn_once!(id: "my_unique_id", "This is a warning that will only be logged once.");
///
/// warn_once!("This warning will also be logged once, but uses the call site as the identifier.");
///```
macro_rules! warn_once {
    // id / name / target / parent.
    (id: $id:expr, name: $name:expr, target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
       $crate::once($id, || tracing::event!(name: $name, target: $target, parent: $parent, tracing::Level::WARN, { $($field)* }, $($arg)*))
    );
    (id: $id:expr, name: $name:expr, target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
       $crate::once($id, || tracing::event!(name: $name, target: $target, parent: $parent, tracing::Level::WARN, { $($k).+ $($field)* }))
    );
    (id: $id:expr, name: $name:expr, target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
       $crate::once($id, || tracing::event!(name: $name, target: $target, parent: $parent, tracing::Level::WARN, { ?$($k).+ $($field)* }))
    );
    (id: $id:expr, name: $name:expr, target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
       $crate::once($id, || tracing::event!(name: $name, target: $target, parent: $parent, tracing::Level::WARN, { %$($k).+ $($field)* }))
    );
    (id: $id:expr, name: $name:expr, target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
       $crate::once($id, || tracing::event!(name: $name, target: $target, parent: $parent, tracing::Level::WARN, {}, $($arg)+))
    );

    // id, name / target.
    (id: $id:expr, name: $name:expr, target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
       $crate::once($id, || tracing::event!(name: $name, target: $target, tracing::Level::WARN, { $($field)* }, $($arg)*))
    );
    (id: $id:expr, name: $name:expr, target: $target:expr, $($k:ident).+ $($field:tt)* ) => (
       $crate::once($id, || tracing::event!(name: $name, target: $target, tracing::Level::WARN, { $($k).+ $($field)* }))
    );
    (id: $id:expr, name: $name:expr, target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => (
       $crate::once($id, || tracing::event!(name: $name, target: $target, tracing::Level::WARN, { ?$($k).+ $($field)* }))
    );
    (id: $id:expr, name: $name:expr, target: $target:expr, %$($k:ident).+ $($field:tt)* ) => (
       $crate::once($id, || tracing::event!(name: $name, target: $target, tracing::Level::WARN, { %$($k).+ $($field)* }))
    );
    (id: $id:expr, name: $name:expr, target: $target:expr, $($arg:tt)+ ) => (
       $crate::once($id, || tracing::event!(name: $name, target: $target, tracing::Level::WARN, {}, $($arg)+))
    );

    // Target / parent.
    (id: $id:expr,target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
       $crate::once($id, || tracing::event!(target: $target, parent: $parent, tracing::Level::WARN, { $($field)* }, $($arg)*))
    );
    (id: $id:expr,target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
       $crate::once($id, || tracing::event!(target: $target, parent: $parent, tracing::Level::WARN, { $($k).+ $($field)* }))
    );
    (id: $id:expr,target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
       $crate::once($id, || tracing::event!(target: $target, parent: $parent, tracing::Level::WARN, { ?$($k).+ $($field)* }))
    );
    (id: $id:expr,target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
       $crate::once($id, || tracing::event!(target: $target, parent: $parent, tracing::Level::WARN, { %$($k).+ $($field)* }))
    );
    (id: $id:expr,target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
       $crate::once($id, || tracing::event!(target: $target, parent: $parent, tracing::Level::WARN, {}, $($arg)+))
    );

    // id / name / parent.
    (id: $id:expr, name: $name:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
       $crate::once($id, || tracing::event!(name: $name, parent: $parent, tracing::Level::WARN, { $($field)* }, $($arg)*))
    );
    (id: $id:expr, name: $name:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
       $crate::once($id, || tracing::event!(name: $name, parent: $parent, tracing::Level::WARN, { $($k).+ $($field)* }))
    );
    (id: $id:expr, name: $name:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
       $crate::once($id, || tracing::event!(name: $name, parent: $parent, tracing::Level::WARN, { ?$($k).+ $($field)* }))
    );
    (id: $id:expr, name: $name:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
       $crate::once($id, || tracing::event!(name: $name, parent: $parent, tracing::Level::WARN, { %$($k).+ $($field)* }))
    );
    (id: $id:expr, name: $name:expr, parent: $parent:expr, $($arg:tt)+ ) => (
       $crate::once($id, || tracing::event!(name: $name, parent: $parent, tracing::Level::WARN, {}, $($arg)+))
    );

    // id / name.
    (id: $id:expr, name: $name:expr, { $($field:tt)* }, $($arg:tt)* ) => (
       $crate::once($id, || tracing::event!(name: $name, tracing::Level::WARN, { $($field)* }, $($arg)*))
    );
    (id: $id:expr, name: $name:expr, $($k:ident).+ $($field:tt)* ) => (
       $crate::once($id, || tracing::event!(name: $name, tracing::Level::WARN, { $($k).+ $($field)* }))
    );
    (id: $id:expr, name: $name:expr, ?$($k:ident).+ $($field:tt)* ) => (
       $crate::once($id, || tracing::event!(name: $name, tracing::Level::WARN, { ?$($k).+ $($field)* }))
    );
    (id: $id:expr, name: $name:expr, %$($k:ident).+ $($field:tt)* ) => (
       $crate::once($id, || tracing::event!(name: $name, tracing::Level::WARN, { %$($k).+ $($field)* }))
    );
    (id: $id:expr, name: $name:expr, $($arg:tt)+ ) => (
       $crate::once($id, || tracing::event!(name: $name, tracing::Level::WARN, {}, $($arg)+))
    );

    // id / target.
    (id: $id:expr,target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
       $crate::once($id, || tracing::event!(target: $target, tracing::Level::WARN, { $($field)* }, $($arg)*))
    );
    (id: $id:expr,target: $target:expr, $($k:ident).+ $($field:tt)* ) => (
       $crate::once($id, || tracing::event!(target: $target, tracing::Level::WARN, { $($k).+ $($field)* }))
    );
    (id: $id:expr,target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => (
       $crate::once($id, || tracing::event!(target: $target, tracing::Level::WARN, { ?$($k).+ $($field)* }))
    );
    (id: $id:expr,target: $target:expr, %$($k:ident).+ $($field:tt)* ) => (
       $crate::once($id, || tracing::event!(target: $target, tracing::Level::WARN, { %$($k).+ $($field)* }))
    );
    (id: $id:expr,target: $target:expr, $($arg:tt)+ ) => (
       $crate::once($id, || tracing::event!(target: $target, tracing::Level::WARN, {}, $($arg)+))
    );

    // id / parent.
    (id: $id:expr, parent: $parent:expr, { $($field:tt)+ }, $($arg:tt)+ ) => (
       $crate::once($id, || tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::WARN,
            { $($field)+ },
            $($arg)+
        ))
    );
    (id: $id:expr, parent: $parent:expr, $($k:ident).+ = $($field:tt)*) => (
       $crate::once($id, || tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::WARN,
            { $($k).+ = $($field)*}
        ))
    );
    (id: $id:expr, parent: $parent:expr, ?$($k:ident).+ = $($field:tt)*) => (
       $crate::once($id, || tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::WARN,
            { ?$($k).+ = $($field)*}
        ))
    );
    (id: $id:expr, parent: $parent:expr, %$($k:ident).+ = $($field:tt)*) => (
       $crate::once($id, || tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::WARN,
            { %$($k).+ = $($field)*}
        ))
    );
    (id: $id:expr, parent: $parent:expr, $($k:ident).+, $($field:tt)*) => (
       $crate::once($id, || tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::WARN,
            { $($k).+, $($field)*}
        ))
    );
    (id: $id:expr, parent: $parent:expr, ?$($k:ident).+, $($field:tt)*) => (
       $crate::once($id, || tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::WARN,
            { ?$($k).+, $($field)*}
        ))
    );
    (id: $id:expr, parent: $parent:expr, %$($k:ident).+, $($field:tt)*) => (
       $crate::once($id, || tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::WARN,
            { %$($k).+, $($field)*}
        ))
    );
    (id: $id:expr, parent: $parent:expr, $($arg:tt)+) => (
       $crate::once($id, || tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::WARN,
            {},
            $($arg)+
        ))
    );

    // id
    (id: $id:expr, { $($field:tt)+ }, $($arg:tt)+ ) => (
       $crate::once($id, || tracing::event!(
            target: module_path!(),
            tracing::Level::WARN,
            { $($field)+ },
            $($arg)+
        ))
    );
    (id: $id:expr, $($k:ident).+ = $($field:tt)*) => (
       $crate::once($id, || tracing::event!(
            target: module_path!(),
            tracing::Level::WARN,
            { $($k).+ = $($field)*}
        ))
    );
    (id: $id:expr, ?$($k:ident).+ = $($field:tt)*) => (
       $crate::once($id, || tracing::event!(
            target: module_path!(),
            tracing::Level::WARN,
            { ?$($k).+ = $($field)*}
        ))
    );
    (id: $id:expr, %$($k:ident).+ = $($field:tt)*) => (
       $crate::once($id, || tracing::event!(
            target: module_path!(),
            tracing::Level::WARN,
            { %$($k).+ = $($field)*}
        ))
    );
    (id: $id:expr, $($k:ident).+, $($field:tt)*) => (
       $crate::once($id, || tracing::event!(
            target: module_path!(),
            tracing::Level::WARN,
            { $($k).+, $($field)*}
        ))
    );
    (id: $id:expr, ?$($k:ident).+, $($field:tt)*) => (
       $crate::once($id, || tracing::event!(
            target: module_path!(),
            tracing::Level::WARN,
            { ?$($k).+, $($field)*}
        ))
    );
    (id: $id:expr, %$($k:ident).+, $($field:tt)*) => (
       $crate::once($id, || tracing::event!(
            target: module_path!(),
            tracing::Level::WARN,
            { %$($k).+, $($field)*}
        ))
    );
    (id: $id:expr, ?$($k:ident).+) => (
       $crate::once($id, || tracing::event!(
            target: module_path!(),
            tracing::Level::WARN,
            { ?$($k).+ }
        ))
    );
    (id: $id:expr, %$($k:ident).+) => (
       $crate::once($id, || tracing::event!(
            target: module_path!(),
            tracing::Level::WARN,
            { %$($k).+ }
        ))
    );
    (id: $id:expr, $($k:ident).+) => (
       $crate::once($id, || tracing::event!(
            target: module_path!(),
            tracing::Level::WARN,
            { $($k).+ }
        ))
    );
    (id: $id:expr, $($arg:tt)+) => (
       $crate::once($id, || tracing::event!(
            target: module_path!(),
            tracing::Level::WARN,
            $($arg)+
        ))
    );

    // Name / target / parent.
    (name: $name:expr, target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
       $crate::once_at_source(|| tracing::event!(name: $name, target: $target, parent: $parent, tracing::Level::WARN, { $($field)* }, $($arg)*))
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
       $crate::once_at_source(||tracing::event!(name: $name, target: $target, parent: $parent, tracing::Level::WARN, { $($k).+ $($field)* }))
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
       $crate::once_at_source(||tracing::event!(name: $name, target: $target, parent: $parent, tracing::Level::WARN, { ?$($k).+ $($field)* }))
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
       $crate::once_at_source(||tracing::event!(name: $name, target: $target, parent: $parent, tracing::Level::WARN, { %$($k).+ $($field)* }))
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
       $crate::once_at_source(||tracing::event!(name: $name, target: $target, parent: $parent, tracing::Level::WARN, {}, $($arg)+))
    );

    // Name / target.
    (name: $name:expr, target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
       $crate::once_at_source(||tracing::event!(name: $name, target: $target, tracing::Level::WARN, { $($field)* }, $($arg)*))
    );
    (name: $name:expr, target: $target:expr, $($k:ident).+ $($field:tt)* ) => (
       $crate::once_at_source(||tracing::event!(name: $name, target: $target, tracing::Level::WARN, { $($k).+ $($field)* }))
    );
    (name: $name:expr, target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => (
       $crate::once_at_source(||tracing::event!(name: $name, target: $target, tracing::Level::WARN, { ?$($k).+ $($field)* }))
    );
    (name: $name:expr, target: $target:expr, %$($k:ident).+ $($field:tt)* ) => (
       $crate::once_at_source(||tracing::event!(name: $name, target: $target, tracing::Level::WARN, { %$($k).+ $($field)* }))
    );
    (name: $name:expr, target: $target:expr, $($arg:tt)+ ) => (
       $crate::once_at_source(||tracing::event!(name: $name, target: $target, tracing::Level::WARN, {}, $($arg)+))
    );

    // Target / parent.
    (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
       $crate::once_at_source(||tracing::event!(target: $target, parent: $parent, tracing::Level::WARN, { $($field)* }, $($arg)*))
    );
    (target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
       $crate::once_at_source(||tracing::event!(target: $target, parent: $parent, tracing::Level::WARN, { $($k).+ $($field)* }))
    );
    (target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
       $crate::once_at_source(||tracing::event!(target: $target, parent: $parent, tracing::Level::WARN, { ?$($k).+ $($field)* }))
    );
    (target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
       $crate::once_at_source(||tracing::event!(target: $target, parent: $parent, tracing::Level::WARN, { %$($k).+ $($field)* }))
    );
    (target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
       $crate::once_at_source(||tracing::event!(target: $target, parent: $parent, tracing::Level::WARN, {}, $($arg)+))
    );

    // Name / parent.
    (name: $name:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
       $crate::once_at_source(||tracing::event!(name: $name, parent: $parent, tracing::Level::WARN, { $($field)* }, $($arg)*))
    );
    (name: $name:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
       $crate::once_at_source(||tracing::event!(name: $name, parent: $parent, tracing::Level::WARN, { $($k).+ $($field)* }))
    );
    (name: $name:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
       $crate::once_at_source(||tracing::event!(name: $name, parent: $parent, tracing::Level::WARN, { ?$($k).+ $($field)* }))
    );
    (name: $name:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
       $crate::once_at_source(||tracing::event!(name: $name, parent: $parent, tracing::Level::WARN, { %$($k).+ $($field)* }))
    );
    (name: $name:expr, parent: $parent:expr, $($arg:tt)+ ) => (
       $crate::once_at_source(||tracing::event!(name: $name, parent: $parent, tracing::Level::WARN, {}, $($arg)+))
    );

    // Name.
    (name: $name:expr, { $($field:tt)* }, $($arg:tt)* ) => (
       $crate::once_at_source(||tracing::event!(name: $name, tracing::Level::WARN, { $($field)* }, $($arg)*))
    );
    (name: $name:expr, $($k:ident).+ $($field:tt)* ) => (
       $crate::once_at_source(||tracing::event!(name: $name, tracing::Level::WARN, { $($k).+ $($field)* }))
    );
    (name: $name:expr, ?$($k:ident).+ $($field:tt)* ) => (
       $crate::once_at_source(||tracing::event!(name: $name, tracing::Level::WARN, { ?$($k).+ $($field)* }))
    );
    (name: $name:expr, %$($k:ident).+ $($field:tt)* ) => (
       $crate::once_at_source(||tracing::event!(name: $name, tracing::Level::WARN, { %$($k).+ $($field)* }))
    );
    (name: $name:expr, $($arg:tt)+ ) => (
       $crate::once_at_source(||tracing::event!(name: $name, tracing::Level::WARN, {}, $($arg)+))
    );

    // Target.
    (target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
       $crate::once_at_source(||tracing::event!(target: $target, tracing::Level::WARN, { $($field)* }, $($arg)*))
    );
    (target: $target:expr, $($k:ident).+ $($field:tt)* ) => (
       $crate::once_at_source(||tracing::event!(target: $target, tracing::Level::WARN, { $($k).+ $($field)* }))
    );
    (target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => (
       $crate::once_at_source(||tracing::event!(target: $target, tracing::Level::WARN, { ?$($k).+ $($field)* }))
    );
    (target: $target:expr, %$($k:ident).+ $($field:tt)* ) => (
       $crate::once_at_source(||tracing::event!(target: $target, tracing::Level::WARN, { %$($k).+ $($field)* }))
    );
    (target: $target:expr, $($arg:tt)+ ) => (
       $crate::once_at_source(||tracing::event!(target: $target, tracing::Level::WARN, {}, $($arg)+))
    );

    // Parent.
    (parent: $parent:expr, { $($field:tt)+ }, $($arg:tt)+ ) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::WARN,
            { $($field)+ },
            $($arg)+
        ))
    );
    (parent: $parent:expr, $($k:ident).+ = $($field:tt)*) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::WARN,
            { $($k).+ = $($field)*}
        ))
    );
    (parent: $parent:expr, ?$($k:ident).+ = $($field:tt)*) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::WARN,
            { ?$($k).+ = $($field)*}
        ))
    );
    (parent: $parent:expr, %$($k:ident).+ = $($field:tt)*) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::WARN,
            { %$($k).+ = $($field)*}
        ))
    );
    (parent: $parent:expr, $($k:ident).+, $($field:tt)*) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::WARN,
            { $($k).+, $($field)*}
        ))
    );
    (parent: $parent:expr, ?$($k:ident).+, $($field:tt)*) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::WARN,
            { ?$($k).+, $($field)*}
        ))
    );
    (parent: $parent:expr, %$($k:ident).+, $($field:tt)*) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::WARN,
            { %$($k).+, $($field)*}
        ))
    );
    (parent: $parent:expr, $($arg:tt)+) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::WARN,
            {},
            $($arg)+
        ))
    );

    // ...
    ({ $($field:tt)+ }, $($arg:tt)+ ) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            tracing::Level::WARN,
            { $($field)+ },
            $($arg)+
        ))
    );
    ($($k:ident).+ = $($field:tt)*) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            tracing::Level::WARN,
            { $($k).+ = $($field)*}
        ))
    );
    (?$($k:ident).+ = $($field:tt)*) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            tracing::Level::WARN,
            { ?$($k).+ = $($field)*}
        ))
    );
    (%$($k:ident).+ = $($field:tt)*) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            tracing::Level::WARN,
            { %$($k).+ = $($field)*}
        ))
    );
    ($($k:ident).+, $($field:tt)*) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            tracing::Level::WARN,
            { $($k).+, $($field)*}
        ))
    );
    (?$($k:ident).+, $($field:tt)*) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            tracing::Level::WARN,
            { ?$($k).+, $($field)*}
        ))
    );
    (%$($k:ident).+, $($field:tt)*) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            tracing::Level::WARN,
            { %$($k).+, $($field)*}
        ))
    );
    (?$($k:ident).+) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            tracing::Level::WARN,
            { ?$($k).+ }
        ))
    );
    (%$($k:ident).+) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            tracing::Level::WARN,
            { %$($k).+ }
        ))
    );
    ($($k:ident).+) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            tracing::Level::WARN,
            { $($k).+ }
        ))
    );
    ($($arg:tt)+) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            tracing::Level::WARN,
            $($arg)+
        ))
    );
}

#[macro_export]
/// Identical to `tracing::error!`, but ensures that the error is logged only once per unique identifier.
/// If no id is provided, it uses the call site as the unique identifier.
///
/// # Example:
/// ```rust
/// use rs_util::error_once;
///
/// error_once!(id: "my_unique_id", "This is an error that will only be logged once.");
///
/// error_once!("This error will also be logged once, but uses the call site as the identifier.");
///```
macro_rules! error_once {
    // id / name / target / parent.
    (id: $id:expr, name: $name:expr, target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
       $crate::once($id, || tracing::event!(name: $name, target: $target, parent: $parent, tracing::Level::ERROR, { $($field)* }, $($arg)*))
    );
    (id: $id:expr, name: $name:expr, target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
       $crate::once($id, || tracing::event!(name: $name, target: $target, parent: $parent, tracing::Level::ERROR, { $($k).+ $($field)* }))
    );
    (id: $id:expr, name: $name:expr, target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
       $crate::once($id, || tracing::event!(name: $name, target: $target, parent: $parent, tracing::Level::ERROR, { ?$($k).+ $($field)* }))
    );
    (id: $id:expr, name: $name:expr, target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
       $crate::once($id, || tracing::event!(name: $name, target: $target, parent: $parent, tracing::Level::ERROR, { %$($k).+ $($field)* }))
    );
    (id: $id:expr, name: $name:expr, target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
       $crate::once($id, || tracing::event!(name: $name, target: $target, parent: $parent, tracing::Level::ERROR, {}, $($arg)+))
    );

    // id, name / target.
    (id: $id:expr, name: $name:expr, target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
       $crate::once($id, || tracing::event!(name: $name, target: $target, tracing::Level::ERROR, { $($field)* }, $($arg)*))
    );
    (id: $id:expr, name: $name:expr, target: $target:expr, $($k:ident).+ $($field:tt)* ) => (
       $crate::once($id, || tracing::event!(name: $name, target: $target, tracing::Level::ERROR, { $($k).+ $($field)* }))
    );
    (id: $id:expr, name: $name:expr, target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => (
       $crate::once($id, || tracing::event!(name: $name, target: $target, tracing::Level::ERROR, { ?$($k).+ $($field)* }))
    );
    (id: $id:expr, name: $name:expr, target: $target:expr, %$($k:ident).+ $($field:tt)* ) => (
       $crate::once($id, || tracing::event!(name: $name, target: $target, tracing::Level::ERROR, { %$($k).+ $($field)* }))
    );
    (id: $id:expr, name: $name:expr, target: $target:expr, $($arg:tt)+ ) => (
       $crate::once($id, || tracing::event!(name: $name, target: $target, tracing::Level::ERROR, {}, $($arg)+))
    );

    // Target / parent.
    (id: $id:expr,target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
       $crate::once($id, || tracing::event!(target: $target, parent: $parent, tracing::Level::ERROR, { $($field)* }, $($arg)*))
    );
    (id: $id:expr,target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
       $crate::once($id, || tracing::event!(target: $target, parent: $parent, tracing::Level::ERROR, { $($k).+ $($field)* }))
    );
    (id: $id:expr,target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
       $crate::once($id, || tracing::event!(target: $target, parent: $parent, tracing::Level::ERROR, { ?$($k).+ $($field)* }))
    );
    (id: $id:expr,target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
       $crate::once($id, || tracing::event!(target: $target, parent: $parent, tracing::Level::ERROR, { %$($k).+ $($field)* }))
    );
    (id: $id:expr,target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
       $crate::once($id, || tracing::event!(target: $target, parent: $parent, tracing::Level::ERROR, {}, $($arg)+))
    );

    // id / name / parent.
    (id: $id:expr, name: $name:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
       $crate::once($id, || tracing::event!(name: $name, parent: $parent, tracing::Level::ERROR, { $($field)* }, $($arg)*))
    );
    (id: $id:expr, name: $name:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
       $crate::once($id, || tracing::event!(name: $name, parent: $parent, tracing::Level::ERROR, { $($k).+ $($field)* }))
    );
    (id: $id:expr, name: $name:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
       $crate::once($id, || tracing::event!(name: $name, parent: $parent, tracing::Level::ERROR, { ?$($k).+ $($field)* }))
    );
    (id: $id:expr, name: $name:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
       $crate::once($id, || tracing::event!(name: $name, parent: $parent, tracing::Level::ERROR, { %$($k).+ $($field)* }))
    );
    (id: $id:expr, name: $name:expr, parent: $parent:expr, $($arg:tt)+ ) => (
       $crate::once($id, || tracing::event!(name: $name, parent: $parent, tracing::Level::ERROR, {}, $($arg)+))
    );

    // id / name.
    (id: $id:expr, name: $name:expr, { $($field:tt)* }, $($arg:tt)* ) => (
       $crate::once($id, || tracing::event!(name: $name, tracing::Level::ERROR, { $($field)* }, $($arg)*))
    );
    (id: $id:expr, name: $name:expr, $($k:ident).+ $($field:tt)* ) => (
       $crate::once($id, || tracing::event!(name: $name, tracing::Level::ERROR, { $($k).+ $($field)* }))
    );
    (id: $id:expr, name: $name:expr, ?$($k:ident).+ $($field:tt)* ) => (
       $crate::once($id, || tracing::event!(name: $name, tracing::Level::ERROR, { ?$($k).+ $($field)* }))
    );
    (id: $id:expr, name: $name:expr, %$($k:ident).+ $($field:tt)* ) => (
       $crate::once($id, || tracing::event!(name: $name, tracing::Level::ERROR, { %$($k).+ $($field)* }))
    );
    (id: $id:expr, name: $name:expr, $($arg:tt)+ ) => (
       $crate::once($id, || tracing::event!(name: $name, tracing::Level::ERROR, {}, $($arg)+))
    );

    // id / target.
    (id: $id:expr,target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
       $crate::once($id, || tracing::event!(target: $target, tracing::Level::ERROR, { $($field)* }, $($arg)*))
    );
    (id: $id:expr,target: $target:expr, $($k:ident).+ $($field:tt)* ) => (
       $crate::once($id, || tracing::event!(target: $target, tracing::Level::ERROR, { $($k).+ $($field)* }))
    );
    (id: $id:expr,target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => (
       $crate::once($id, || tracing::event!(target: $target, tracing::Level::ERROR, { ?$($k).+ $($field)* }))
    );
    (id: $id:expr,target: $target:expr, %$($k:ident).+ $($field:tt)* ) => (
       $crate::once($id, || tracing::event!(target: $target, tracing::Level::ERROR, { %$($k).+ $($field)* }))
    );
    (id: $id:expr,target: $target:expr, $($arg:tt)+ ) => (
       $crate::once($id, || tracing::event!(target: $target, tracing::Level::ERROR, {}, $($arg)+))
    );

    // id / parent.
    (id: $id:expr, parent: $parent:expr, { $($field:tt)+ }, $($arg:tt)+ ) => (
       $crate::once($id, || tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::ERROR,
            { $($field)+ },
            $($arg)+
        ))
    );
    (id: $id:expr, parent: $parent:expr, $($k:ident).+ = $($field:tt)*) => (
       $crate::once($id, || tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::ERROR,
            { $($k).+ = $($field)*}
        ))
    );
    (id: $id:expr, parent: $parent:expr, ?$($k:ident).+ = $($field:tt)*) => (
       $crate::once($id, || tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::ERROR,
            { ?$($k).+ = $($field)*}
        ))
    );
    (id: $id:expr, parent: $parent:expr, %$($k:ident).+ = $($field:tt)*) => (
       $crate::once($id, || tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::ERROR,
            { %$($k).+ = $($field)*}
        ))
    );
    (id: $id:expr, parent: $parent:expr, $($k:ident).+, $($field:tt)*) => (
       $crate::once($id, || tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::ERROR,
            { $($k).+, $($field)*}
        ))
    );
    (id: $id:expr, parent: $parent:expr, ?$($k:ident).+, $($field:tt)*) => (
       $crate::once($id, || tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::ERROR,
            { ?$($k).+, $($field)*}
        ))
    );
    (id: $id:expr, parent: $parent:expr, %$($k:ident).+, $($field:tt)*) => (
       $crate::once($id, || tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::ERROR,
            { %$($k).+, $($field)*}
        ))
    );
    (id: $id:expr, parent: $parent:expr, $($arg:tt)+) => (
       $crate::once($id, || tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::ERROR,
            {},
            $($arg)+
        ))
    );

    // id
    (id: $id:expr, { $($field:tt)+ }, $($arg:tt)+ ) => (
       $crate::once($id, || tracing::event!(
            target: module_path!(),
            tracing::Level::ERROR,
            { $($field)+ },
            $($arg)+
        ))
    );
    (id: $id:expr, $($k:ident).+ = $($field:tt)*) => (
       $crate::once($id, || tracing::event!(
            target: module_path!(),
            tracing::Level::ERROR,
            { $($k).+ = $($field)*}
        ))
    );
    (id: $id:expr, ?$($k:ident).+ = $($field:tt)*) => (
       $crate::once($id, || tracing::event!(
            target: module_path!(),
            tracing::Level::ERROR,
            { ?$($k).+ = $($field)*}
        ))
    );
    (id: $id:expr, %$($k:ident).+ = $($field:tt)*) => (
       $crate::once($id, || tracing::event!(
            target: module_path!(),
            tracing::Level::ERROR,
            { %$($k).+ = $($field)*}
        ))
    );
    (id: $id:expr, $($k:ident).+, $($field:tt)*) => (
       $crate::once($id, || tracing::event!(
            target: module_path!(),
            tracing::Level::ERROR,
            { $($k).+, $($field)*}
        ))
    );
    (id: $id:expr, ?$($k:ident).+, $($field:tt)*) => (
       $crate::once($id, || tracing::event!(
            target: module_path!(),
            tracing::Level::ERROR,
            { ?$($k).+, $($field)*}
        ))
    );
    (id: $id:expr, %$($k:ident).+, $($field:tt)*) => (
       $crate::once($id, || tracing::event!(
            target: module_path!(),
            tracing::Level::ERROR,
            { %$($k).+, $($field)*}
        ))
    );
    (id: $id:expr, ?$($k:ident).+) => (
       $crate::once($id, || tracing::event!(
            target: module_path!(),
            tracing::Level::ERROR,
            { ?$($k).+ }
        ))
    );
    (id: $id:expr, %$($k:ident).+) => (
       $crate::once($id, || tracing::event!(
            target: module_path!(),
            tracing::Level::ERROR,
            { %$($k).+ }
        ))
    );
    (id: $id:expr, $($k:ident).+) => (
       $crate::once($id, || tracing::event!(
            target: module_path!(),
            tracing::Level::ERROR,
            { $($k).+ }
        ))
    );
    (id: $id:expr, $($arg:tt)+) => (
       $crate::once($id, || tracing::event!(
            target: module_path!(),
            tracing::Level::ERROR,
            $($arg)+
        ))
    );

    // Name / target / parent.
    (name: $name:expr, target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
       $crate::once_at_source(|| tracing::event!(name: $name, target: $target, parent: $parent, tracing::Level::ERROR, { $($field)* }, $($arg)*))
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
       $crate::once_at_source(||tracing::event!(name: $name, target: $target, parent: $parent, tracing::Level::ERROR, { $($k).+ $($field)* }))
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
       $crate::once_at_source(||tracing::event!(name: $name, target: $target, parent: $parent, tracing::Level::ERROR, { ?$($k).+ $($field)* }))
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
       $crate::once_at_source(||tracing::event!(name: $name, target: $target, parent: $parent, tracing::Level::ERROR, { %$($k).+ $($field)* }))
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
       $crate::once_at_source(||tracing::event!(name: $name, target: $target, parent: $parent, tracing::Level::ERROR, {}, $($arg)+))
    );

    // Name / target.
    (name: $name:expr, target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
       $crate::once_at_source(||tracing::event!(name: $name, target: $target, tracing::Level::ERROR, { $($field)* }, $($arg)*))
    );
    (name: $name:expr, target: $target:expr, $($k:ident).+ $($field:tt)* ) => (
       $crate::once_at_source(||tracing::event!(name: $name, target: $target, tracing::Level::ERROR, { $($k).+ $($field)* }))
    );
    (name: $name:expr, target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => (
       $crate::once_at_source(||tracing::event!(name: $name, target: $target, tracing::Level::ERROR, { ?$($k).+ $($field)* }))
    );
    (name: $name:expr, target: $target:expr, %$($k:ident).+ $($field:tt)* ) => (
       $crate::once_at_source(||tracing::event!(name: $name, target: $target, tracing::Level::ERROR, { %$($k).+ $($field)* }))
    );
    (name: $name:expr, target: $target:expr, $($arg:tt)+ ) => (
       $crate::once_at_source(||tracing::event!(name: $name, target: $target, tracing::Level::ERROR, {}, $($arg)+))
    );

    // Target / parent.
    (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
       $crate::once_at_source(||tracing::event!(target: $target, parent: $parent, tracing::Level::ERROR, { $($field)* }, $($arg)*))
    );
    (target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
       $crate::once_at_source(||tracing::event!(target: $target, parent: $parent, tracing::Level::ERROR, { $($k).+ $($field)* }))
    );
    (target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
       $crate::once_at_source(||tracing::event!(target: $target, parent: $parent, tracing::Level::ERROR, { ?$($k).+ $($field)* }))
    );
    (target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
       $crate::once_at_source(||tracing::event!(target: $target, parent: $parent, tracing::Level::ERROR, { %$($k).+ $($field)* }))
    );
    (target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
       $crate::once_at_source(||tracing::event!(target: $target, parent: $parent, tracing::Level::ERROR, {}, $($arg)+))
    );

    // Name / parent.
    (name: $name:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
       $crate::once_at_source(||tracing::event!(name: $name, parent: $parent, tracing::Level::ERROR, { $($field)* }, $($arg)*))
    );
    (name: $name:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
       $crate::once_at_source(||tracing::event!(name: $name, parent: $parent, tracing::Level::ERROR, { $($k).+ $($field)* }))
    );
    (name: $name:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
       $crate::once_at_source(||tracing::event!(name: $name, parent: $parent, tracing::Level::ERROR, { ?$($k).+ $($field)* }))
    );
    (name: $name:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
       $crate::once_at_source(||tracing::event!(name: $name, parent: $parent, tracing::Level::ERROR, { %$($k).+ $($field)* }))
    );
    (name: $name:expr, parent: $parent:expr, $($arg:tt)+ ) => (
       $crate::once_at_source(||tracing::event!(name: $name, parent: $parent, tracing::Level::ERROR, {}, $($arg)+))
    );

    // Name.
    (name: $name:expr, { $($field:tt)* }, $($arg:tt)* ) => (
       $crate::once_at_source(||tracing::event!(name: $name, tracing::Level::ERROR, { $($field)* }, $($arg)*))
    );
    (name: $name:expr, $($k:ident).+ $($field:tt)* ) => (
       $crate::once_at_source(||tracing::event!(name: $name, tracing::Level::ERROR, { $($k).+ $($field)* }))
    );
    (name: $name:expr, ?$($k:ident).+ $($field:tt)* ) => (
       $crate::once_at_source(||tracing::event!(name: $name, tracing::Level::ERROR, { ?$($k).+ $($field)* }))
    );
    (name: $name:expr, %$($k:ident).+ $($field:tt)* ) => (
       $crate::once_at_source(||tracing::event!(name: $name, tracing::Level::ERROR, { %$($k).+ $($field)* }))
    );
    (name: $name:expr, $($arg:tt)+ ) => (
       $crate::once_at_source(||tracing::event!(name: $name, tracing::Level::ERROR, {}, $($arg)+))
    );

    // Target.
    (target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
       $crate::once_at_source(||tracing::event!(target: $target, tracing::Level::ERROR, { $($field)* }, $($arg)*))
    );
    (target: $target:expr, $($k:ident).+ $($field:tt)* ) => (
       $crate::once_at_source(||tracing::event!(target: $target, tracing::Level::ERROR, { $($k).+ $($field)* }))
    );
    (target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => (
       $crate::once_at_source(||tracing::event!(target: $target, tracing::Level::ERROR, { ?$($k).+ $($field)* }))
    );
    (target: $target:expr, %$($k:ident).+ $($field:tt)* ) => (
       $crate::once_at_source(||tracing::event!(target: $target, tracing::Level::ERROR, { %$($k).+ $($field)* }))
    );
    (target: $target:expr, $($arg:tt)+ ) => (
       $crate::once_at_source(||tracing::event!(target: $target, tracing::Level::ERROR, {}, $($arg)+))
    );

    // Parent.
    (parent: $parent:expr, { $($field:tt)+ }, $($arg:tt)+ ) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::ERROR,
            { $($field)+ },
            $($arg)+
        ))
    );
    (parent: $parent:expr, $($k:ident).+ = $($field:tt)*) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::ERROR,
            { $($k).+ = $($field)*}
        ))
    );
    (parent: $parent:expr, ?$($k:ident).+ = $($field:tt)*) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::ERROR,
            { ?$($k).+ = $($field)*}
        ))
    );
    (parent: $parent:expr, %$($k:ident).+ = $($field:tt)*) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::ERROR,
            { %$($k).+ = $($field)*}
        ))
    );
    (parent: $parent:expr, $($k:ident).+, $($field:tt)*) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::ERROR,
            { $($k).+, $($field)*}
        ))
    );
    (parent: $parent:expr, ?$($k:ident).+, $($field:tt)*) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::ERROR,
            { ?$($k).+, $($field)*}
        ))
    );
    (parent: $parent:expr, %$($k:ident).+, $($field:tt)*) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::ERROR,
            { %$($k).+, $($field)*}
        ))
    );
    (parent: $parent:expr, $($arg:tt)+) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::ERROR,
            {},
            $($arg)+
        ))
    );

    // ...
    ({ $($field:tt)+ }, $($arg:tt)+ ) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            tracing::Level::ERROR,
            { $($field)+ },
            $($arg)+
        ))
    );
    ($($k:ident).+ = $($field:tt)*) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            tracing::Level::ERROR,
            { $($k).+ = $($field)*}
        ))
    );
    (?$($k:ident).+ = $($field:tt)*) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            tracing::Level::ERROR,
            { ?$($k).+ = $($field)*}
        ))
    );
    (%$($k:ident).+ = $($field:tt)*) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            tracing::Level::ERROR,
            { %$($k).+ = $($field)*}
        ))
    );
    ($($k:ident).+, $($field:tt)*) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            tracing::Level::ERROR,
            { $($k).+, $($field)*}
        ))
    );
    (?$($k:ident).+, $($field:tt)*) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            tracing::Level::ERROR,
            { ?$($k).+, $($field)*}
        ))
    );
    (%$($k:ident).+, $($field:tt)*) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            tracing::Level::ERROR,
            { %$($k).+, $($field)*}
        ))
    );
    (?$($k:ident).+) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            tracing::Level::ERROR,
            { ?$($k).+ }
        ))
    );
    (%$($k:ident).+) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            tracing::Level::ERROR,
            { %$($k).+ }
        ))
    );
    ($($k:ident).+) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            tracing::Level::ERROR,
            { $($k).+ }
        ))
    );
    ($($arg:tt)+) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            tracing::Level::ERROR,
            $($arg)+
        ))
    );
}

#[macro_export]
/// Identical to `tracing::info!`, but ensures that the info is logged only once per unique identifier.
/// If no id is provided, it uses the call site as the unique identifier.
///
/// # Example:
/// ```rust
/// use rs_util::info_once;
///
/// info_once!(id: "my_unique_id", "This is an info that will only be logged once.");
///
/// info_once!("This info will also be logged once, but uses the call site as the identifier.");
///```
macro_rules! info_once {
    // id / name / target / parent.
    (id: $id:expr, name: $name:expr, target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::once($id, || tracing::event!(name: $name, target: $target, parent: $parent, tracing::Level::INFO, { $($field)* }, $($arg)*))
    );
    (id: $id:expr, name: $name:expr, target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::once($id, || tracing::event!(name: $name, target: $target, parent: $parent, tracing::Level::INFO, { $($k).+ $($field)* }))
    );
    (id: $id:expr, name: $name:expr, target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::once($id, || tracing::event!(name: $name, target: $target, parent: $parent, tracing::Level::INFO, { ?$($k).+ $($field)* }))
    );
    (id: $id:expr, name: $name:expr, target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::once($id, || tracing::event!(name: $name, target: $target, parent: $parent, tracing::Level::INFO, { %$($k).+ $($field)* }))
    );
    (id: $id:expr, name: $name:expr, target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
        $crate::once($id, || tracing::event!(name: $name, target: $target, parent: $parent, tracing::Level::INFO, {}, $($arg)+))
    );

    // id, name / target.
    (id: $id:expr, name: $name:expr, target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::once($id, || tracing::event!(name: $name, target: $target, tracing::Level::INFO, { $($field)* }, $($arg)*))
    );
    (id: $id:expr, name: $name:expr, target: $target:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::once($id, || tracing::event!(name: $name, target: $target, tracing::Level::INFO, { $($k).+ $($field)* }))
    );
    (id: $id:expr, name: $name:expr, target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::once($id, || tracing::event!(name: $name, target: $target, tracing::Level::INFO, { ?$($k).+ $($field)* }))
    );
    (id: $id:expr, name: $name:expr, target: $target:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::once($id, || tracing::event!(name: $name, target: $target, tracing::Level::INFO, { %$($k).+ $($field)* }))
    );
    (id: $id:expr, name: $name:expr, target: $target:expr, $($arg:tt)+ ) => (
        $crate::once($id, || tracing::event!(name: $name, target: $target, tracing::Level::INFO, {}, $($arg)+))
    );

    // Target / parent.
    (id: $id:expr,target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::once($id, || tracing::event!(target: $target, parent: $parent, tracing::Level::INFO, { $($field)* }, $($arg)*))
    );
    (id: $id:expr,target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::once($id, || tracing::event!(target: $target, parent: $parent, tracing::Level::INFO, { $($k).+ $($field)* }))
    );
    (id: $id:expr,target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::once($id, || tracing::event!(target: $target, parent: $parent, tracing::Level::INFO, { ?$($k).+ $($field)* }))
    );
    (id: $id:expr,target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::once($id, || tracing::event!(target: $target, parent: $parent, tracing::Level::INFO, { %$($k).+ $($field)* }))
    );
    (id: $id:expr,target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
        $crate::once($id, || tracing::event!(target: $target, parent: $parent, tracing::Level::INFO, {}, $($arg)+))
    );

    // id / name / parent.
    (id: $id:expr, name: $name:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::once($id, || tracing::event!(name: $name, parent: $parent, tracing::Level::INFO, { $($field)* }, $($arg)*))
    );
    (id: $id:expr, name: $name:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::once($id, || tracing::event!(name: $name, parent: $parent, tracing::Level::INFO, { $($k).+ $($field)* }))
    );
    (id: $id:expr, name: $name:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::once($id, || tracing::event!(name: $name, parent: $parent, tracing::Level::INFO, { ?$($k).+ $($field)* }))
    );
    (id: $id:expr, name: $name:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::once($id, || tracing::event!(name: $name, parent: $parent, tracing::Level::INFO, { %$($k).+ $($field)* }))
    );
    (id: $id:expr, name: $name:expr, parent: $parent:expr, $($arg:tt)+ ) => (
        $crate::once($id, || tracing::event!(name: $name, parent: $parent, tracing::Level::INFO, {}, $($arg)+))
    );

    // id / name.
    (id: $id:expr, name: $name:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::once($id, || tracing::event!(name: $name, tracing::Level::INFO, { $($field)* }, $($arg)*))
    );
    (id: $id:expr, name: $name:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::once($id, || tracing::event!(name: $name, tracing::Level::INFO, { $($k).+ $($field)* }))
    );
    (id: $id:expr, name: $name:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::once($id, || tracing::event!(name: $name, tracing::Level::INFO, { ?$($k).+ $($field)* }))
    );
    (id: $id:expr, name: $name:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::once($id, || tracing::event!(name: $name, tracing::Level::INFO, { %$($k).+ $($field)* }))
    );
    (id: $id:expr, name: $name:expr, $($arg:tt)+ ) => (
        $crate::once($id, || tracing::event!(name: $name, tracing::Level::INFO, {}, $($arg)+))
    );

    // id / target.
    (id: $id:expr,target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::once($id, || tracing::event!(target: $target, tracing::Level::INFO, { $($field)* }, $($arg)*))
    );
    (id: $id:expr,target: $target:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::once($id, || tracing::event!(target: $target, tracing::Level::INFO, { $($k).+ $($field)* }))
    );
    (id: $id:expr,target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::once($id, || tracing::event!(target: $target, tracing::Level::INFO, { ?$($k).+ $($field)* }))
    );
    (id: $id:expr,target: $target:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::once($id, || tracing::event!(target: $target, tracing::Level::INFO, { %$($k).+ $($field)* }))
    );
    (id: $id:expr,target: $target:expr, $($arg:tt)+ ) => (
        $crate::once($id, || tracing::event!(target: $target, tracing::Level::INFO, {}, $($arg)+))
    );

    // id / parent.
    (id: $id:expr, parent: $parent:expr, { $($field:tt)+ }, $($arg:tt)+ ) => (
        $crate::once($id, || tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::INFO,
            { $($field)+ },
            $($arg)+
        ))
    );
    (id: $id:expr, parent: $parent:expr, $($k:ident).+ = $($field:tt)*) => (
        $crate::once($id, || tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::INFO,
            { $($k).+ = $($field)*}
        ))
    );
    (id: $id:expr, parent: $parent:expr, ?$($k:ident).+ = $($field:tt)*) => (
        $crate::once($id, || tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::INFO,
            { ?$($k).+ = $($field)*}
        ))
    );
    (id: $id:expr, parent: $parent:expr, %$($k:ident).+ = $($field:tt)*) => (
        $crate::once($id, || tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::INFO,
            { %$($k).+ = $($field)*}
        ))
    );
    (id: $id:expr, parent: $parent:expr, $($k:ident).+, $($field:tt)*) => (
        $crate::once($id, || tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::INFO,
            { $($k).+, $($field)*}
        ))
    );
    (id: $id:expr, parent: $parent:expr, ?$($k:ident).+, $($field:tt)*) => (
        $crate::once($id, || tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::INFO,
            { ?$($k).+, $($field)*}
        ))
    );
    (id: $id:expr, parent: $parent:expr, %$($k:ident).+, $($field:tt)*) => (
        $crate::once($id, || tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::INFO,
            { %$($k).+, $($field)*}
        ))
    );
    (id: $id:expr, parent: $parent:expr, $($arg:tt)+) => (
        $crate::once($id, || tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::INFO,
            {},
            $($arg)+
        ))
    );

    // id
    (id: $id:expr, { $($field:tt)+ }, $($arg:tt)+ ) => (
        $crate::once($id, || tracing::event!(
            target: module_path!(),
            tracing::Level::INFO,
            { $($field)+ },
            $($arg)+
        ))
    );
    (id: $id:expr, $($k:ident).+ = $($field:tt)*) => (
        $crate::once($id, || tracing::event!(
            target: module_path!(),
            tracing::Level::INFO,
            { $($k).+ = $($field)*}
        ))
    );
    (id: $id:expr, ?$($k:ident).+ = $($field:tt)*) => (
        $crate::once($id, || tracing::event!(
            target: module_path!(),
            tracing::Level::INFO,
            { ?$($k).+ = $($field)*}
        ))
    );
    (id: $id:expr, %$($k:ident).+ = $($field:tt)*) => (
        $crate::once($id, || tracing::event!(
            target: module_path!(),
            tracing::Level::INFO,
            { %$($k).+ = $($field)*}
        ))
    );
    (id: $id:expr, $($k:ident).+, $($field:tt)*) => (
        $crate::once($id, || tracing::event!(
            target: module_path!(),
            tracing::Level::INFO,
            { $($k).+, $($field)*}
        ))
    );
    (id: $id:expr, ?$($k:ident).+, $($field:tt)*) => (
        $crate::once($id, || tracing::event!(
            target: module_path!(),
            tracing::Level::INFO,
            { ?$($k).+, $($field)*}
        ))
    );
    (id: $id:expr, %$($k:ident).+, $($field:tt)*) => (
        $crate::once($id, || tracing::event!(
            target: module_path!(),
            tracing::Level::INFO,
            { %$($k).+, $($field)*}
        ))
    );
    (id: $id:expr, ?$($k:ident).+) => (
        $crate::once($id, || tracing::event!(
            target: module_path!(),
            tracing::Level::INFO,
            { ?$($k).+ }
        ))
    );
    (id: $id:expr, %$($k:ident).+) => (
        $crate::once($id, || tracing::event!(
            target: module_path!(),
            tracing::Level::INFO,
            { %$($k).+ }
        ))
    );
    (id: $id:expr, $($k:ident).+) => (
        $crate::once($id, || tracing::event!(
            target: module_path!(),
            tracing::Level::INFO,
            { $($k).+ }
        ))
    );
    (id: $id:expr, $($arg:tt)+) => (
        $crate::once($id, || tracing::event!(
            target: module_path!(),
            tracing::Level::INFO,
            $($arg)+
        ))
    );

    // Name / target / parent.
    (name: $name:expr, target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::once_at_source(|| tracing::event!(name: $name, target: $target, parent: $parent, tracing::Level::INFO, { $($field)* }, $($arg)*))
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::once_at_source(||tracing::event!(name: $name, target: $target, parent: $parent, tracing::Level::INFO, { $($k).+ $($field)* }))
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::once_at_source(||tracing::event!(name: $name, target: $target, parent: $parent, tracing::Level::INFO, { ?$($k).+ $($field)* }))
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::once_at_source(||tracing::event!(name: $name, target: $target, parent: $parent, tracing::Level::INFO, { %$($k).+ $($field)* }))
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
        $crate::once_at_source(||tracing::event!(name: $name, target: $target, parent: $parent, tracing::Level::INFO, {}, $($arg)+))
    );

    // Name / target.
    (name: $name:expr, target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::once_at_source(||tracing::event!(name: $name, target: $target, tracing::Level::INFO, { $($field)* }, $($arg)*))
    );
    (name: $name:expr, target: $target:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::once_at_source(||tracing::event!(name: $name, target: $target, tracing::Level::INFO, { $($k).+ $($field)* }))
    );
    (name: $name:expr, target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::once_at_source(||tracing::event!(name: $name, target: $target, tracing::Level::INFO, { ?$($k).+ $($field)* }))
    );
    (name: $name:expr, target: $target:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::once_at_source(||tracing::event!(name: $name, target: $target, tracing::Level::INFO, { %$($k).+ $($field)* }))
    );
    (name: $name:expr, target: $target:expr, $($arg:tt)+ ) => (
        $crate::once_at_source(||tracing::event!(name: $name, target: $target, tracing::Level::INFO, {}, $($arg)+))
    );

    // Target / parent.
    (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::once_at_source(||tracing::event!(target: $target, parent: $parent, tracing::Level::INFO, { $($field)* }, $($arg)*))
    );
    (target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::once_at_source(||tracing::event!(target: $target, parent: $parent, tracing::Level::INFO, { $($k).+ $($field)* }))
    );
    (target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::once_at_source(||tracing::event!(target: $target, parent: $parent, tracing::Level::INFO, { ?$($k).+ $($field)* }))
    );
    (target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::once_at_source(||tracing::event!(target: $target, parent: $parent, tracing::Level::INFO, { %$($k).+ $($field)* }))
    );
    (target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
        $crate::once_at_source(||tracing::event!(target: $target, parent: $parent, tracing::Level::INFO, {}, $($arg)+))
    );

    // Name / parent.
    (name: $name:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::once_at_source(||tracing::event!(name: $name, parent: $parent, tracing::Level::INFO, { $($field)* }, $($arg)*))
    );
    (name: $name:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::once_at_source(||tracing::event!(name: $name, parent: $parent, tracing::Level::INFO, { $($k).+ $($field)* }))
    );
    (name: $name:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::once_at_source(||tracing::event!(name: $name, parent: $parent, tracing::Level::INFO, { ?$($k).+ $($field)* }))
    );
    (name: $name:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::once_at_source(||tracing::event!(name: $name, parent: $parent, tracing::Level::INFO, { %$($k).+ $($field)* }))
    );
    (name: $name:expr, parent: $parent:expr, $($arg:tt)+ ) => (
        $crate::once_at_source(||tracing::event!(name: $name, parent: $parent, tracing::Level::INFO, {}, $($arg)+))
    );

    // Name.
    (name: $name:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::once_at_source(||tracing::event!(name: $name, tracing::Level::INFO, { $($field)* }, $($arg)*))
    );
    (name: $name:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::once_at_source(||tracing::event!(name: $name, tracing::Level::INFO, { $($k).+ $($field)* }))
    );
    (name: $name:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::once_at_source(||tracing::event!(name: $name, tracing::Level::INFO, { ?$($k).+ $($field)* }))
    );
    (name: $name:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::once_at_source(||tracing::event!(name: $name, tracing::Level::INFO, { %$($k).+ $($field)* }))
    );
    (name: $name:expr, $($arg:tt)+ ) => (
        $crate::once_at_source(||tracing::event!(name: $name, tracing::Level::INFO, {}, $($arg)+))
    );

    // Target.
    (target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::once_at_source(||tracing::event!(target: $target, tracing::Level::INFO, { $($field)* }, $($arg)*))
    );
    (target: $target:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::once_at_source(||tracing::event!(target: $target, tracing::Level::INFO, { $($k).+ $($field)* }))
    );
    (target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::once_at_source(||tracing::event!(target: $target, tracing::Level::INFO, { ?$($k).+ $($field)* }))
    );
    (target: $target:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::once_at_source(||tracing::event!(target: $target, tracing::Level::INFO, { %$($k).+ $($field)* }))
    );
    (target: $target:expr, $($arg:tt)+ ) => (
        $crate::once_at_source(||tracing::event!(target: $target, tracing::Level::INFO, {}, $($arg)+))
    );

    // Parent.
    (parent: $parent:expr, { $($field:tt)+ }, $($arg:tt)+ ) => (
        $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::INFO,
            { $($field)+ },
            $($arg)+
        ))
    );
    (parent: $parent:expr, $($k:ident).+ = $($field:tt)*) => (
        $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::INFO,
            { $($k).+ = $($field)*}
        ))
    );
    (parent: $parent:expr, ?$($k:ident).+ = $($field:tt)*) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::INFO,
            { ?$($k).+ = $($field)*}
        ))
    );
    (parent: $parent:expr, %$($k:ident).+ = $($field:tt)*) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::INFO,
            { %$($k).+ = $($field)*}
        ))
    );
    (parent: $parent:expr, $($k:ident).+, $($field:tt)*) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::INFO,
            { $($k).+, $($field)*}
        ))
    );
    (parent: $parent:expr, ?$($k:ident).+, $($field:tt)*) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::INFO,
            { ?$($k).+, $($field)*}
        ))
    );
    (parent: $parent:expr, %$($k:ident).+, $($field:tt)*) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::INFO,
            { %$($k).+, $($field)*}
        ))
    );
    (parent: $parent:expr, $($arg:tt)+) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            parent: $parent,
            tracing::Level::INFO,
            {},
            $($arg)+
        ))
    );

    // ...
    ({ $($field:tt)+ }, $($arg:tt)+ ) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            tracing::Level::INFO,
            { $($field)+ },
            $($arg)+
        ))
    );
    ($($k:ident).+ = $($field:tt)*) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            tracing::Level::INFO,
            { $($k).+ = $($field)*}
        ))
    );
    (?$($k:ident).+ = $($field:tt)*) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            tracing::Level::INFO,
            { ?$($k).+ = $($field)*}
        ))
    );
    (%$($k:ident).+ = $($field:tt)*) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            tracing::Level::INFO,
            { %$($k).+ = $($field)*}
        ))
    );
    ($($k:ident).+, $($field:tt)*) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            tracing::Level::INFO,
            { $($k).+, $($field)*}
        ))
    );
    (?$($k:ident).+, $($field:tt)*) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            tracing::Level::INFO,
            { ?$($k).+, $($field)*}
        ))
    );
    (%$($k:ident).+, $($field:tt)*) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            tracing::Level::INFO,
            { %$($k).+, $($field)*}
        ))
    );
    (?$($k:ident).+) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            tracing::Level::INFO,
            { ?$($k).+ }
        ))
    );
    (%$($k:ident).+) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            tracing::Level::INFO,
            { %$($k).+ }
        ))
    );
    ($($k:ident).+) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            tracing::Level::INFO,
            { $($k).+ }
        ))
    );
    ($($arg:tt)+) => (
       $crate::once_at_source(||tracing::event!(
            target: module_path!(),
            tracing::Level::INFO,
            $($arg)+
        ))
    );
}
