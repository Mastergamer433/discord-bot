mod getvotes;
mod vote;
mod help;
mod main;

use crate::structs::Command;
use getvotes::*;
use vote::*;
use help::*;

pub fn commands() -> Vec<Command> {
    main::commands().into_iter()
	.chain(getvotes::commands())
	.chain(vote::commands())
	.chain(help::commands())
	.collect()
}
