// Copyright 2023 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", autorest_diff::say_hello(None));
    Ok(())
}
