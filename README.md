# Holo
_Basic HTTP server for local hosting._  
__Copyright &copy; 2024 Chris Roberts__ (Krobbizoid).

# Contents
1. [Usage](#usage)
2. [Dependencies](#dependencies)
3. [License](#license)

# Usage
Holo is a basic HTTP server targeted at serving local files for testing. The
original motivation for this project was to gain lower-level access to HTTP
headers so that cross-origin isolation can be tested.

Using cross-origin isolation on a local server may not be allowed in some
browsers without adjusting security settings.

# Dependencies
Holo uses the following libraries:
* [percent-encoding](https://crates.io/crates/percent-encoding) - Request path
decoding.

# License
Holo is released under the MIT License:  
https://krobbi.github.io/license/2024/mit.txt

See [LICENSE.txt](/LICENSE.txt) for a full copy of the license text.
