// Miniscript
// Written in 2019 by
//     Andrew Poelstra <apoelstra@wpsoftware.net>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! Example: Parsing a descriptor from a string

extern crate bitcoin;
extern crate miniscript;

use miniscript::NullCtx;
use std::str::FromStr;

fn main() {
    let my_descriptor = miniscript::Descriptor::<bitcoin::PublicKey>::from_str(
        "wsh(c:pk_k(020202020202020202020202020202020202020202020202020202020202020202))",
    )
    .unwrap();

    // Sometimes it is necessary to have additional information to get the bitcoin::PublicKey
    // from the MiniscriptKey which can supplied by `to_pk_ctx` parameter. For example,
    // when calculating the script pubkey of a descriptor with xpubs, the secp context and
    // child information maybe required.
    assert_eq!(
        format!("{:x}", my_descriptor.script_pubkey(NullCtx)),
        "0020daef16dd7c946a3e735a6e43310cb2ce33dfd14a04f76bf8241a16654cb2f0f9"
    );

    assert_eq!(
        format!("{:x}", my_descriptor.witness_script(NullCtx)),
        "21020202020202020202020202020202020202020202020202020202020202020202ac"
    );
}
