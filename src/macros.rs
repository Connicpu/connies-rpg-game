macro_rules! def_system {
    (pub struct $systy:ident;) => {
        pub struct $systy;
        impl ::ecs::System for $systy {
            type Services = ::Services;
            type Components = ::Components;
        }
    };

    (
        pub struct $systy:ident;
        fn process(&mut self, $procdata:ident : &mut DataHelper) $procbody:block
    ) => {
        def_system!(pub struct $systy;);
        impl ::ecs::Process for $systy {
            fn process(&mut self, $procdata : &mut ::DataHelper) {
                $procbody;
            }
        }
    };
}
