#[cfg(not(any(feature = "free", feature = "disposable")))]
compile_error!("You must enable at least one of `free` and `disposable` features!");

#[cfg(feature = "free")]
include!(concat!(env!("OUT_DIR"), "/free.rs"));

/// Checks if the domain is for a free email provider.
#[cfg(feature = "free")]
#[must_use]
pub fn is_free_domain(domain: &str) -> bool {
    FREE_EMAILS.contains(domain.to_ascii_lowercase().trim().as_bytes())
}

/// Checks if the email is from a free email provider.
#[cfg(feature = "free")]
#[must_use]
pub fn is_free_email(email: &str) -> bool {
    let domain = extract_domain_from_email(email);
    is_free_domain(domain)
}

#[cfg(feature = "disposable")]
include!(concat!(env!("OUT_DIR"), "/disposable.rs"));

/// Checks if the domain is for a disposable email provider.
#[cfg(feature = "disposable")]
#[must_use]
pub fn is_disposable_domain(domain: &str) -> bool {
    DISPOSABLE_EMAILS.contains(domain.to_ascii_lowercase().trim().as_bytes())
}

/// Checks if the email is from a disposable email provider.
#[cfg(feature = "disposable")]
#[must_use]
pub fn is_disposable_email(email: &str) -> bool {
    let domain = extract_domain_from_email(email);
    is_disposable_domain(domain)
}

/// Checks if the domain is for a work email.
#[must_use]
pub fn is_work_domain(domain: &str) -> bool {
    #[cfg(feature = "free")]
    if is_free_domain(domain) {
        return false;
    }

    #[cfg(feature = "disposable")]
    if is_disposable_domain(domain) {
        return false;
    }

    true
}

/// Checks if the email is a work email.
#[must_use]
pub fn is_work_email(email: &str) -> bool {
    let domain = extract_domain_from_email(email);
    is_work_domain(domain)
}

#[inline]
fn extract_domain_from_email(email: &str) -> &str {
    let mut email = email.split('@');
    email.nth(1).unwrap_or(email.next().unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "free")]
    fn test_is_free_domain() {
        assert!(is_free_domain("GMaIl.COM"));
        assert!(is_free_domain(" GMaIl.COM "));
    }

    #[test]
    #[cfg(feature = "free")]
    fn test_is_free_email() {
        assert!(is_free_email("test@GMaIl.COM"));
        assert!(is_free_email(" test@GMaIl.COM "));
    }

    #[test]
    #[cfg(feature = "disposable")]
    fn test_is_disposable_domain() {
        assert!(is_disposable_domain("MaiLinator.com"));
        assert!(is_disposable_domain(" maiLinAtor.com "));
    }

    #[test]
    #[cfg(feature = "disposable")]
    fn test_is_disposable_email() {
        assert!(is_disposable_email("test@MaiLinator.com"));
        assert!(is_disposable_email(" test@maiLinAtor.com "));
    }

    #[test]
    fn test_is_work_domain() {
        assert!(!is_work_domain("MaiLinator.com"));
        assert!(!is_work_domain("GMaIl.COM"));
    }

    #[test]
    fn test_is_work_email() {
        assert!(!is_work_email("test@MaiLinator.com"));
        assert!(!is_work_email("test@GMaIl.COM"));
    }
}
