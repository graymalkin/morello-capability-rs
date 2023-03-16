use crate::capability::caps_eq_exact;

trait CapabilityPtr<T> {
    fn eq(&self, other: *const T) -> bool;
}

impl<T> CapabilityPtr<T> for *const T {
    fn eq(&self, other: *const T) -> bool {
        return caps_eq_exact(*self, other)
    }
}
