use std::borrow::Cow;

// Regex wrapper class that allows for unified management
// of invalid regex patterns
#[derive(Clone)]
pub struct Regex(Option<regex::Regex>);

impl Regex {
    pub fn new(re: &str) -> Result<Regex, regex::Error> {
        let regex = match regex::Regex::new(re) {
            Ok(r) => Some(r),
            Err(_) if cfg!(feature = "ignore-invalid") => None,
            Err(e) => return Err(e),
        };
        Ok(Regex(regex))
    }

    #[inline(always)]
    pub fn find<'h>(&self, haystack: &'h str) -> Option<regex::Match<'h>> {
        self.0.as_ref().and_then(|r| r.find(haystack))
    }

    #[inline(always)]
    pub fn find_at<'h>(&self, haystack: &'h str, start: usize) -> Option<regex::Match<'h>> {
        self.0.as_ref().and_then(|r| r.find_at(haystack, start))
    }

    #[inline(always)]
    pub fn find_iter<'r, 'h>(&'r self, haystack: &'h str) -> OptionIter<regex::Matches<'r, 'h>> {
        self.0.as_ref().map_or_else(OptionIter::none, |r| {
            OptionIter::some(r.find_iter(haystack))
        })
    }

    #[inline(always)]
    pub fn split<'r, 'h>(&'r self, haystack: &'h str) -> OptionIter<regex::Split<'r, 'h>>
    where
        'h: 'r,
    {
        self.0
            .as_ref()
            .map_or(OptionIter::none(), |r| OptionIter::some(r.split(haystack)))
    }

    #[inline(always)]
    pub fn as_str(&self) -> &str {
        self.0.as_ref().map_or("invalid", |r| r.as_str())
    }

    #[inline(always)]
    pub fn is_match(&self, haystack: &str) -> bool {
        self.0.as_ref().map_or(false, |r| r.is_match(haystack))
    }

    #[inline(always)]
    pub fn replace<'h, R: regex::Replacer>(&self, haystack: &'h str, rep: R) -> Cow<'h, str> {
        self.0
            .as_ref()
            .map_or_else(|| Cow::Borrowed(haystack), |r| r.replace(haystack, rep))
    }

    #[inline(always)]
    pub fn replace_all<'h, R: regex::Replacer>(&self, haystack: &'h str, rep: R) -> Cow<'h, str> {
        self.0
            .as_ref()
            .map_or_else(|| Cow::Borrowed(haystack), |r| r.replace_all(haystack, rep))
    }

    #[inline(always)]
    pub fn captures<'h>(&self, haystack: &'h str) -> Option<regex::Captures<'h>> {
        self.0.as_ref().and_then(|r| r.captures(haystack))
    }

    #[inline(always)]
    pub fn captures_iter<'r, 'h>(
        &'r self,
        haystack: &'h str,
    ) -> OptionIter<regex::CaptureMatches<'r, 'h>> {
        self.0.as_ref().map_or_else(OptionIter::none, |r| {
            OptionIter::some(r.captures_iter(haystack))
        })
    }

    #[inline(always)]
    pub fn capture_names(&self) -> OptionIter<regex::CaptureNames> {
        self.0
            .as_ref()
            .map_or_else(OptionIter::none, |r| OptionIter::some(r.capture_names()))
    }
}

// RegexSet wrapper class that allows for unified management
// of invalid regex patterns
#[derive(Clone)]
pub struct RegexSet(Option<regex::RegexSet>);

impl RegexSet {
    pub fn new<I, S>(exprs: I) -> Result<RegexSet, regex::Error>
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {
        let regex = match regex::RegexSet::new(exprs) {
            Ok(r) => Some(r),
            Err(_) if cfg!(feature = "ignore-invalid") => None,
            Err(e) => return Err(e),
        };
        Ok(RegexSet(regex))
    }

    #[inline(always)]
    pub fn patterns(&self) -> &[String] {
        self.0.as_ref().map_or(&[][..], |r| r.patterns())
    }

    #[inline(always)]
    pub fn is_match(&self, haystack: &str) -> bool {
        self.0.as_ref().map_or(false, |r| r.is_match(haystack))
    }

    #[inline(always)]
    pub fn matches(&self, haystack: &str) -> OptionIntoIter<regex::SetMatches> {
        self.0.as_ref().map_or_else(OptionIntoIter::none, |r| {
            OptionIntoIter::some(r.matches(haystack))
        })
    }
}

pub struct OptionIter<T: Iterator>(Option<T>);

impl<T: Iterator> OptionIter<T> {
    #[inline(always)]
    pub fn some(value: T) -> Self {
        Self(Some(value))
    }

    #[inline(always)]
    pub fn none() -> Self {
        Self(None)
    }
}

impl<T: Iterator> Iterator for OptionIter<T> {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.as_mut()?.next()
    }
}

pub struct OptionIntoIter<T: IntoIterator>(Option<T>);

impl<T: IntoIterator> OptionIntoIter<T> {
    #[inline(always)]
    pub fn some(value: T) -> Self {
        Self(Some(value))
    }

    #[inline(always)]
    pub fn none() -> Self {
        Self(None)
    }
}

impl<T: IntoIterator> IntoIterator for OptionIntoIter<T> {
    type Item = T::Item;
    type IntoIter = OptionIter<T::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        OptionIter(self.0.map(IntoIterator::into_iter))
    }
}
