pub fn spawn<F>(f: F)
    where F: FnOnce(),
          F: Send + 'static
{
    rayon::spawn(move || {
        f();
    });
}