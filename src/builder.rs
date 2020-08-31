/*
 * Copyright © 2020 Peter M. Stahl pemistahl@gmail.com
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either expressed or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::detector::LanguageDetector;
use crate::isocode::{IsoCode639_1, IsoCode639_3};
use crate::language::Language;
use std::collections::HashSet;

const MISSING_LANGUAGE_MESSAGE: &str = "LanguageDetector needs at least 2 languages to choose from";

pub struct LanguageDetectorBuilder {
    languages: HashSet<Language>,
    minimum_relative_distance: f64,
}

impl LanguageDetectorBuilder {
    pub fn from_all_languages() -> Self {
        Self::from(Language::all())
    }

    pub fn from_all_spoken_languages() -> Self {
        Self::from(Language::all_spoken_ones())
    }

    pub fn from_all_languages_with_arabic_script() -> Self {
        Self::from(Language::all_with_arabic_script())
    }

    pub fn from_all_languages_with_cyrillic_script() -> Self {
        Self::from(Language::all_with_cyrillic_script())
    }

    pub fn from_all_languages_with_devanagari_script() -> Self {
        Self::from(Language::all_with_devanagari_script())
    }

    pub fn from_all_languages_with_latin_script() -> Self {
        Self::from(Language::all_with_latin_script())
    }

    pub fn from_all_languages_without(languages: &[Language]) -> Self {
        let mut languages_to_load = Language::all();
        languages_to_load.retain(|it| !languages.contains(it));
        if languages_to_load.len() < 2 {
            panic!(MISSING_LANGUAGE_MESSAGE);
        }
        Self::from(languages_to_load)
    }

    pub fn from_languages(languages: &[Language]) -> Self {
        if languages.len() < 2 {
            panic!(MISSING_LANGUAGE_MESSAGE);
        }
        Self::from(languages.iter().cloned().collect())
    }

    pub fn from_iso_codes_639_1(iso_codes: &[IsoCode639_1]) -> Self {
        if iso_codes.len() < 2 {
            panic!(MISSING_LANGUAGE_MESSAGE);
        }
        let languages = iso_codes
            .iter()
            .map(|it| Language::from_iso_code_639_1(it))
            .collect::<HashSet<_>>();
        Self::from(languages)
    }

    pub fn from_iso_codes_639_3(iso_codes: &[IsoCode639_3]) -> Self {
        if iso_codes.len() < 2 {
            panic!(MISSING_LANGUAGE_MESSAGE);
        }
        let languages = iso_codes
            .iter()
            .map(|it| Language::from_iso_code_639_3(it))
            .collect::<HashSet<_>>();
        Self::from(languages)
    }

    pub fn with_minimum_relative_distance(&mut self, distance: f64) -> &mut Self {
        if distance < 0.0 || distance > 0.99 {
            panic!("minimum relative distance must lie in between 0.0 and 0.99");
        }
        self.minimum_relative_distance = distance;
        self
    }

    pub fn build(&mut self) -> LanguageDetector {
        LanguageDetector::from(self.languages.clone(), self.minimum_relative_distance)
    }

    fn from(languages: HashSet<Language>) -> Self {
        Self {
            languages,
            minimum_relative_distance: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_detector_can_be_built_from_all_languages() {
        let mut builder = LanguageDetectorBuilder::from_all_languages();
        assert_eq!(builder.languages, Language::all());
        assert_eq!(builder.minimum_relative_distance, 0.0);

        builder.with_minimum_relative_distance(0.2);
        assert_eq!(builder.minimum_relative_distance, 0.2);
    }

    #[test]
    fn assert_detector_can_be_built_from_spoken_languages() {
        let mut builder = LanguageDetectorBuilder::from_all_spoken_languages();
        assert_eq!(builder.languages, Language::all_spoken_ones());
        assert_eq!(builder.minimum_relative_distance, 0.0);

        builder.with_minimum_relative_distance(0.2);
        assert_eq!(builder.minimum_relative_distance, 0.2);
    }

    #[test]
    fn assert_detector_can_be_built_from_languages_with_arabic_script() {
        let builder = LanguageDetectorBuilder::from_all_languages_with_arabic_script();
        assert_eq!(builder.languages, Language::all_with_arabic_script());
    }

    #[test]
    fn assert_detector_can_be_built_from_languages_with_cyrillic_script() {
        let builder = LanguageDetectorBuilder::from_all_languages_with_cyrillic_script();
        assert_eq!(builder.languages, Language::all_with_cyrillic_script());
    }

    #[test]
    fn assert_detector_can_be_built_from_languages_with_devanagari_script() {
        let builder = LanguageDetectorBuilder::from_all_languages_with_devanagari_script();
        assert_eq!(builder.languages, Language::all_with_devanagari_script());
    }

    #[test]
    fn assert_detector_can_be_built_from_languages_with_latin_script() {
        let builder = LanguageDetectorBuilder::from_all_languages_with_latin_script();
        assert_eq!(builder.languages, Language::all_with_latin_script());
    }

    #[test]
    fn assert_detector_can_be_built_from_blacklist() {
        let builder = LanguageDetectorBuilder::from_all_languages_without(&[
            Language::Turkish,
            Language::Romanian,
        ]);
        let expected_languages = Language::all()
            .difference(&hashset!(Language::Turkish, Language::Romanian))
            .cloned()
            .collect::<HashSet<Language>>();

        assert_eq!(builder.languages, expected_languages);
    }

    #[test]
    #[should_panic(expected = "LanguageDetector needs at least 2 languages to choose from")]
    fn assert_detector_cannot_be_built_from_too_long_blacklist() {
        let languages = Language::all()
            .difference(&hashset!(Language::German))
            .cloned()
            .collect::<Vec<_>>();

        LanguageDetectorBuilder::from_all_languages_without(&languages);
    }

    #[test]
    fn assert_detector_can_be_built_from_whitelist() {
        let builder =
            LanguageDetectorBuilder::from_languages(&[Language::German, Language::English]);

        assert_eq!(
            builder.languages,
            hashset!(Language::German, Language::English)
        );
    }

    #[test]
    #[should_panic(expected = "LanguageDetector needs at least 2 languages to choose from")]
    fn assert_detector_cannot_be_built_from_too_short_whitelist() {
        LanguageDetectorBuilder::from_languages(&[Language::German]);
    }

    #[test]
    fn assert_detector_can_be_built_from_iso_639_1_codes() {
        let builder =
            LanguageDetectorBuilder::from_iso_codes_639_1(&[IsoCode639_1::DE, IsoCode639_1::SV]);

        assert_eq!(
            builder.languages,
            hashset!(Language::German, Language::Swedish)
        );
    }

    #[test]
    #[should_panic(expected = "LanguageDetector needs at least 2 languages to choose from")]
    fn assert_detector_cannot_be_built_from_too_few_iso_639_1_codes() {
        LanguageDetectorBuilder::from_iso_codes_639_1(&[IsoCode639_1::DE]);
    }

    #[test]
    fn assert_detector_can_be_built_from_iso_639_3_codes() {
        let builder =
            LanguageDetectorBuilder::from_iso_codes_639_3(&[IsoCode639_3::DEU, IsoCode639_3::SWE]);

        assert_eq!(
            builder.languages,
            hashset!(Language::German, Language::Swedish)
        );
    }

    #[test]
    #[should_panic(expected = "LanguageDetector needs at least 2 languages to choose from")]
    fn assert_detector_cannot_be_built_from_too_few_iso_639_3_codes() {
        LanguageDetectorBuilder::from_iso_codes_639_3(&[IsoCode639_3::DEU]);
    }

    #[test]
    #[should_panic(expected = "minimum relative distance must lie in between 0.0 and 0.99")]
    fn assert_detector_cannot_be_built_from_too_small_minimum_relative_distance() {
        LanguageDetectorBuilder::from_all_languages().with_minimum_relative_distance(-2.3);
    }

    #[test]
    #[should_panic(expected = "minimum relative distance must lie in between 0.0 and 0.99")]
    fn assert_detector_cannot_be_built_from_too_large_minimum_relative_distance() {
        LanguageDetectorBuilder::from_all_languages().with_minimum_relative_distance(1.7);
    }
}
