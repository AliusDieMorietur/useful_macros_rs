macro_rules! import {
	($($path:path),* ) => {
		$(use $path;)*
	};
}

macro_rules! s {
	($s:tt) => {
		String::from($s)
	};
}
