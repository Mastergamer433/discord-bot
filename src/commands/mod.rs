mod getvotes;
mod vote;
mod help;
mod main;
mod points;

use crate::structs::Command;
use getvotes::*;
use vote::*;
use help::*;
use points::*;

pub fn commands() -> Vec<Command> {
    main::commands().into_iter()
	.chain(getvotes::commands())
	.chain(vote::commands())
	.chain(help::commands())
	.chain(points::commands())
	.collect()
}
