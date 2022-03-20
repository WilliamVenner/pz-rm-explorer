#[macro_export]
#[cfg(debug_assertions)]
macro_rules! pz_path {
	($path:literal) => {
		concat!(r#"C:\Program Files (x86)\Steam\steamapps\common\Project Zomboid Dedicated Server\"#, $path)
	};
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! pz_path {
	($path:literal) => {
		concat!("/tmp/pz-rm-explorer/", $path)
	};
}

#[macro_export]
macro_rules! or_continue {
	($expr:expr) => {
		match crate::util::OrContinue::__or_continue($expr) {
			Some(v) => v,
			None => continue
		}
	};

	($expr:expr; ?? $err:expr) => {
		match crate::util::OrContinue::__or_continue($expr) {
			Some(v) => v,
			None => {
				$err;
				continue
			}
		}
	};
}

#[doc(hidden)]
pub trait OrContinue<V>: Sized {
	#[doc(hidden)]
	fn __or_continue(self) -> Option<V>;
}
impl<V> OrContinue<V> for Option<V> {
	#[doc(hidden)]
	#[inline(always)]
	fn __or_continue(self) -> Option<V> {
		self
	}
}
impl<V, E> OrContinue<V> for Result<V, E> {
	#[doc(hidden)]
	#[inline(always)]
	fn __or_continue(self) -> Option<V> {
		self.ok()
	}
}