macro_rules! def_system {
    (
        #[noprocess]
        pub struct $systy:ident;
    ) => {
        
        pub struct $systy;
        impl ::ecs::System for $systy {
            type Services = ::Services;
            type Components = ::Components;
        }
    };

    (
        pub struct $systy:ident;
    ) => {
        pub struct $systy;
        impl ::ecs::System for $systy {
            type Services = ::Services;
            type Components = ::Components;
        }
        impl ::ecs::Process for $systy {
            fn process(&mut self, data : &mut ::DataHelper) {
                process(data);
            }
        }
    };

    (
        #[entity]
        #[aspect(all: [$($all_field:ident),*], none: [$($none_field:ident),*])]
        pub struct $systy:ident;
    ) => {
        def_system!(#[noprocess] pub struct $systy;);
        impl $systy {
            pub fn new() -> ::ecs::system::EntitySystem< $systy > {
                ::ecs::system::EntitySystem::new( $systy, aspect!(<::Components> all: [$($all_field),*] none: [$($none_field),*]) )
            }
        }
        impl ::ecs::system::EntityProcess for $systy {
            fn process<'a>(&mut self, entities: EntityIter<'a, ::Components>, data: &mut ::DataHelper) {
                process(entities, data);
            }
        }
    };

    (
        #[entity]
        #[aspect(all: [$($all_field:ident),*])]
        pub struct $systy:ident;
    ) => {
        def_system!(#[noprocess] pub struct $systy;);
        impl $systy {
            pub fn new() -> ::ecs::system::EntitySystem< $systy > {
                ::ecs::system::EntitySystem::new( $systy, aspect!(<::Components> all: [$($all_field),*]) )
            }
        }
        impl ::ecs::system::EntityProcess for $systy {
            fn process<'a>(&mut self, entities: EntityIter<'a, ::Components>, data: &mut ::DataHelper) {
                process(entities, data);
            }
        }
    };

    (
        #[entity]
        #[aspect(none: [$($none_field:ident),*])]
        pub struct $systy:ident;
    ) => {
        def_system!(#[noprocess] pub struct $systy;);
        impl $systy {
            pub fn new() -> ::ecs::system::EntitySystem< $systy > {
                ::ecs::system::EntitySystem::new( $systy, aspect!(<::Components> none: [$($none_field),*]) )
            }
        }
        impl ::ecs::system::EntityProcess for $systy {
            fn process<'a>(&mut self, entities: EntityIter<'a, ::Components>, data: &mut ::DataHelper) {
                process(entities, data);
            }
        }
    };
}
