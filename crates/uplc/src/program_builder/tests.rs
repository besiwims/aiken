use super::*;
use crate::parser;
use crate::program_builder::constant::WithConstant;
use proptest::prelude::*;

prop_compose! {
    fn arb_version()(
        maj: isize,
        min: isize,
        patch: isize,
    ) -> (usize, usize, usize){
        let maj = maj.abs() as usize;
        let min = min.abs() as usize;
        let patch = patch.abs() as usize;
        (maj, min, patch)
    }
}

proptest! {
    #[test]
    fn build_named__with_version(
        (maj, min, patch) in arb_version(),
    ) {
        let code = format!(r"(program
                           {}.{}.{}
                           (con integer 11)
                         )", maj, min, patch);
        let expected = parser::program(&code).unwrap();
        let actual = Builder::start(maj, min, patch).with_int(11).build_named();
        assert_eq!(expected, actual);
    }
}
