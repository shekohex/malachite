use std::collections::HashMap;
use std::marker::PhantomData;

pub const SMALL_LIMIT: usize = 1000;
pub const LARGE_LIMIT: usize = 10000;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum GenMode {
    Exhaustive,
    Random,
    SpecialRandom,
}

impl GenMode {
    pub const fn name(self) -> &'static str {
        match self {
            GenMode::Exhaustive => "exhaustive",
            GenMode::Random => "random",
            GenMode::SpecialRandom => "special_random",
        }
    }
}

pub type It<T> = Box<dyn Iterator<Item = T>>;

#[derive(Clone, Debug)]
pub struct GenConfig(HashMap<String, u64>);

impl GenConfig {
    pub fn new() -> GenConfig {
        GenConfig(HashMap::new())
    }

    pub fn insert(&mut self, key: String, value: u64) {
        self.0.insert(key, value);
    }

    pub fn get_or(&self, key: &'static str, default: u64) -> u64 {
        *self.0.get(key).unwrap_or(&default)
    }
}

impl Default for GenConfig {
    fn default() -> GenConfig {
        GenConfig::new()
    }
}

pub struct Generator<T: 'static> {
    phantom_data: PhantomData<*const T>,
    exhaustive: &'static dyn Fn() -> It<T>,
    random: &'static dyn Fn(&GenConfig) -> It<T>,
    special_random: Option<&'static dyn Fn(&GenConfig) -> It<T>>,
}

impl<T> Generator<T> {
    pub fn new(
        exhaustive: &'static dyn Fn() -> It<T>,
        random: &'static dyn Fn(&GenConfig) -> It<T>,
        special_random: &'static dyn Fn(&GenConfig) -> It<T>,
    ) -> Generator<T> {
        Generator {
            phantom_data: PhantomData,
            exhaustive,
            random,
            special_random: Some(special_random),
        }
    }

    pub fn new_no_special(
        exhaustive: &'static dyn Fn() -> It<T>,
        random: &'static dyn Fn(&GenConfig) -> It<T>,
    ) -> Generator<T> {
        Generator {
            phantom_data: PhantomData,
            exhaustive,
            random,
            special_random: None,
        }
    }

    fn test_properties_with_config_optional_exhaustive_limit<F: FnMut(T)>(
        &self,
        config: &GenConfig,
        mut test: F,
        exhaustive_limit: bool,
    ) {
        if exhaustive_limit {
            for x in (self.exhaustive)().take(LARGE_LIMIT) {
                test(x);
            }
        } else {
            for x in (self.exhaustive)() {
                test(x);
            }
        }
        for x in (self.random)(config).take(LARGE_LIMIT) {
            test(x);
        }
        if let Some(special_random) = self.special_random {
            for x in special_random(config).take(LARGE_LIMIT) {
                test(x);
            }
        }
    }

    pub fn test_properties_with_config<F: FnMut(T)>(&self, config: &GenConfig, test: F) {
        self.test_properties_with_config_optional_exhaustive_limit(config, test, true);
    }

    #[inline]
    pub fn test_properties<F: FnMut(T)>(&self, test: F) {
        self.test_properties_with_config(&GenConfig::new(), test);
    }

    #[inline]
    pub fn test_properties_no_exhaustive_limit<F: FnMut(T)>(&self, test: F) {
        self.test_properties_with_config_optional_exhaustive_limit(&GenConfig::new(), test, false);
    }

    pub fn get(&self, gm: GenMode, config: &GenConfig) -> It<T> {
        match gm {
            GenMode::Exhaustive => (self.exhaustive)(),
            GenMode::Random => (self.random)(config),
            GenMode::SpecialRandom => {
                (self
                    .special_random
                    .expect("special_random mode unsupported"))(config)
            }
        }
    }
}
