mod getvotes;
mod vote;
mod help;
mod main;
mod points;
mod manage;
mod options;

use crate::structs::Command;

pub fn commands() -> Vec<Command> {
    main::commands().into_iter()
	.chain(getvotes::commands())
	.chain(vote::commands())
	.chain(help::commands())
	.chain(points::commands())
	.chain(manage::commands())
	.chain(options::commands())
	.collect()
}
