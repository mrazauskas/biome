use crate::configuration::overrides::OverrideFormatterConfiguration;
use crate::settings::{to_matcher, FormatSettings};
use crate::{Matcher, WorkspaceError};
use biome_deserialize::StringSet;
use biome_deserialize_macros::{Merge, NoneState};
use biome_formatter::{IndentStyle, LineEnding, LineWidth};
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::str::FromStr;

/// Generic options applied to all files
#[derive(Bpaf, Clone, Debug, Deserialize, Eq, Merge, NoneState, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct FormatterConfiguration {
    // if `false`, it disables the feature. `true` by default
    #[bpaf(hide)]
    pub enabled: Option<bool>,

    /// Stores whether formatting should be allowed to proceed if a given file
    /// has syntax errors
    #[bpaf(hide)]
    pub format_with_errors: Option<bool>,

    /// The indent style.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("indent-style"), argument("tab|space"), optional)]
    pub indent_style: Option<PlainIndentStyle>,

    /// The size of the indentation, 2 by default (deprecated, use `indent-width`)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("indent-size"), argument("NUMBER"), optional)]
    pub indent_size: Option<u8>,

    /// The size of the indentation, 2 by default
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("indent-width"), argument("NUMBER"), optional)]
    pub indent_width: Option<u8>,

    /// The type of line ending.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("line-ending"), argument("lf|crlf|cr"), optional)]
    pub line_ending: Option<LineEnding>,

    /// What's the max width of a line. Defaults to 80.
    #[serde(
        deserialize_with = "deserialize_line_width",
        serialize_with = "serialize_line_width"
    )]
    #[bpaf(long("line-width"), argument("NUMBER"), optional)]
    pub line_width: Option<LineWidth>,

    /// A list of Unix shell style patterns. The formatter will ignore files/folders that will
    /// match these patterns.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(hide)]
    pub ignore: Option<StringSet>,

    /// A list of Unix shell style patterns. The formatter will include files/folders that will
    /// match these patterns.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(hide)]
    pub include: Option<StringSet>,
}

impl FormatterConfiguration {
    pub const fn is_disabled(&self) -> bool {
        matches!(self.enabled, Some(false))
    }
}

impl Default for FormatterConfiguration {
    fn default() -> Self {
        Self {
            enabled: Some(true),
            format_with_errors: Some(false),
            indent_size: Some(2),
            indent_width: Some(2),
            indent_style: Some(PlainIndentStyle::default()),
            line_ending: Some(LineEnding::default()),
            line_width: Some(LineWidth::default()),
            ignore: None,
            include: None,
        }
    }
}

/// Required by [Bpaf].
impl FromStr for FormatterConfiguration {
    type Err = &'static str;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(Self::default())
    }
}

pub fn to_format_settings(
    working_directory: Option<PathBuf>,
    conf: FormatterConfiguration,
    _vcs_path: Option<PathBuf>,
    _gitignore_matches: &[String],
) -> Result<FormatSettings, WorkspaceError> {
    let indent_style = match conf.indent_style {
        Some(PlainIndentStyle::Tab) => IndentStyle::Tab,
        Some(PlainIndentStyle::Space) => IndentStyle::Space,
        None => IndentStyle::default(),
    };
    let indent_width = conf
        .indent_width
        .map(Into::into)
        .or(conf.indent_size.map(Into::into))
        .unwrap_or_default();

    Ok(FormatSettings {
        enabled: conf.enabled.unwrap_or_default(),
        indent_style: Some(indent_style),
        indent_width: Some(indent_width),
        line_ending: conf.line_ending,
        line_width: conf.line_width,
        format_with_errors: conf.format_with_errors.unwrap_or_default(),
        ignored_files: to_matcher(working_directory.clone(), conf.ignore.as_ref(), None, &[])?,
        included_files: to_matcher(working_directory, conf.include.as_ref(), None, &[])?,
    })
}

impl TryFrom<OverrideFormatterConfiguration> for FormatSettings {
    type Error = WorkspaceError;

    fn try_from(conf: OverrideFormatterConfiguration) -> Result<Self, Self::Error> {
        let indent_style = match conf.indent_style {
            Some(PlainIndentStyle::Tab) => IndentStyle::Tab,
            Some(PlainIndentStyle::Space) => IndentStyle::Space,
            None => IndentStyle::default(),
        };
        let indent_width = conf
            .indent_width
            .map(Into::into)
            .or(conf.indent_size.map(Into::into))
            .unwrap_or_default();

        Ok(Self {
            enabled: conf.enabled.unwrap_or_default(),
            indent_style: Some(indent_style),
            indent_width: Some(indent_width),
            line_ending: conf.line_ending,
            line_width: conf.line_width,
            format_with_errors: conf.format_with_errors.unwrap_or_default(),
            ignored_files: Matcher::empty(),
            included_files: Matcher::empty(),
        })
    }
}

impl From<PlainIndentStyle> for IndentStyle {
    fn from(value: PlainIndentStyle) -> Self {
        match value {
            PlainIndentStyle::Tab => IndentStyle::Tab,
            PlainIndentStyle::Space => IndentStyle::Space,
        }
    }
}

pub fn deserialize_line_width<'de, D>(deserializer: D) -> Result<Option<LineWidth>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let value: u16 = Deserialize::deserialize(deserializer)?;
    let line_width = LineWidth::try_from(value).map_err(serde::de::Error::custom)?;
    Ok(Some(line_width))
}

pub fn serialize_line_width<S>(line_width: &Option<LineWidth>, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::ser::Serializer,
{
    s.serialize_u16(line_width.unwrap_or_default().get())
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum PlainIndentStyle {
    /// Tab
    #[default]
    Tab,
    /// Space
    Space,
}

impl FromStr for PlainIndentStyle {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tab" => Ok(PlainIndentStyle::Tab),
            "space" => Ok(PlainIndentStyle::Space),
            _ => Err("Unsupported value for this option"),
        }
    }
}
