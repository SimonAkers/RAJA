/// Macro for generating widget convenience getter
#[macro_export]
macro_rules! widget {
    ($widget:ident, $t:ty) => {
        #[inline(always)]
        pub fn $widget(&self) -> $t {
            self.imp().$widget.get()
        }
    }
}