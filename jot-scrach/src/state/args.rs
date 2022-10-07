use crate::enums::{ConfigType, Item, VaultItem};
use clap::{AppSettings, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(global_setting(AppSettings::HidePossibleValuesInHelp))]
#[clap(global_setting(AppSettings::DontCollapseArgsInUsage))]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
#[clap(global_setting(AppSettings::ColorNever))]
#[clap(help_template("\x1b[0;34m________      _____
______(_)_______  /_
_____  /_  __ \\  __/
____  / / /_/ / /_
___  /  \\____/\\__/
/___/
\x1b[0m

\x1b[0;34mv0.1.1\x1b[0m | crafted with ❤️ by \x1b[0;34maraekiel\x1b[0m


usage: jt <command>

\x1b[0;34mcommands:\x1b[0m

create items
    \x1b[0;34mvault\x1b[0m, \x1b[0;34mvl\x1b[0m       create a vault or list vaults
    create items in current folder
        \x1b[0;34mnote\x1b[0m, \x1b[0;34mnt\x1b[0m        create a note
        \x1b[0;34mfolder\x1b[0m, \x1b[0;34mfd\x1b[0m      create a folder

interact with items
    \x1b[0;34menter\x1b[0m, \x1b[0;34men\x1b[0m       enter a vault
    \x1b[0;34mopen\x1b[0m, \x1b[0;34mop\x1b[0m        open a note from current folder
    \x1b[0;34mchdir\x1b[0m, \x1b[0;34mcd\x1b[0m       change folder within current vault
    \x1b[0;34mlist\x1b[0m, \x1b[0;34mls\x1b[0m        print dir tree of current folder

perform fs operations on items
    \x1b[0;34mremove\x1b[0m, \x1b[0;34mrm\x1b[0m      remove an item
    \x1b[0;34mrename\x1b[0m, \x1b[0;34mrn\x1b[0m      rename an item
    \x1b[0;34mmove\x1b[0m, \x1b[0;34mmv\x1b[0m        move an item to a new location
    \x1b[0;34mvmove\x1b[0m, \x1b[0;34mvm\x1b[0m       move an item to a different vault

config
    \x1b[0;34mconfig\x1b[0m, \x1b[0;34mcf\x1b[0m      set and get config values

get help
    use \x1b[0;34mhelp\x1b[0m or \x1b[0;34m-h\x1b[0m and \x1b[0;34m--help\x1b[0m flags along with a command to get corresponding help"))]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[clap(override_usage(
        "jt vault\n    jt vault -l\n    jt vault <vault name> <vault location>"
    ))]
    #[clap(alias = "vl")]
    Vault {
        #[clap(parse(from_flag), short = 'l')]
        show_loc: bool,
        #[clap(value_parser, name = "vault name", requires = "vault location")]
        name: Option<String>,
        #[clap(value_parser, name = "vault location")]
        location: Option<PathBuf>,
    },
    #[clap(alias = "en")]
    Enter {
        #[clap(value_parser, name = "vault name")]
        name: String,
    },

    #[clap(override_usage("jt note\n    jt note [note name]"))]
    #[clap(alias = "nt")]
    Note {
        #[clap(value_parser, name = "note name")]
        name: String,
    },
    #[clap(alias = "op")]
    Open {
        #[clap(value_parser, name = "note name")]
        name: String,
    },
    #[clap(override_usage("jt folder\n .   jt folder [folder name"))]
    #[clap(alias = "fd")]
    Folder {
        #[clap(value_parser, name = "folder name")]
        name: String,
    },
    #[clap(alias = "cd")]
    Chdir {
        #[clap(value_parser, name = "folder path")]
        path: PathBuf,
    },
    #[clap(alias = "rm")]
    Remove {
        #[clap(value_enum, value_parser, name = "item type")]
        item_type: Item,
        #[clap(value_parser, name = "name")]
        name: String,
    },
    Rename {
        #[clap(value_enum, value_parser, name = "item type")]
        item_type: Item,
        #[clap(value_parser, name = "name")]
        name: String,
        #[clap(value_parser, name = "new name")]
        new_name: String,
    },
    Move {
        #[clap(value_enum, value_parser, name = "item type")]
        item_type: Item,
        #[clap(value_parser, name = "name")]
        name: String,
        #[clap(value_parser, name = "new location")]
        new_location: PathBuf,
    },
    #[clap(alias = "vm")]
    Vmove {
        #[clap(value_enum, value_parser, name = "item type")]
        item_type: VaultItem,
        #[clap(value_parser, name = "name")]
        name: String,
        #[clap(value_parser, name = "valut name")]
        vault_name: String,
    },
    #[clap(alias = "ls")]
    List,
    #[clap(override_usage(
        "jt config <config type>\n .   jt config <config type> [config value]"
    ))]
    #[clap(alias = "cf")]
    Config {
        #[clap(value_enum, value_parser, name = "config type")]
        config_type: ConfigType,
        #[clap(value_parser, name = "config value")]
        value: Option<String>,
    },
    Help,
}
