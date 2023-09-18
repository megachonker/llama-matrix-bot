# llama-matrix-bot

A Matrix chat Bot using LLama.cpp.

it do not use binding or lib it use directly fork of llama.cpp process, it been choosen because i dont wont to learn another lib.

## command
```
!help
!start 
!stop
!reset 
```
## capability
it can handle multiple room and it robust resource exhaustion.
it lunch with a originial number of worker that is givan 

- handle room dynamically
- multiple user
- reset prompt history
- schedule LLAMA worker
- multiple profile of worker
- [ ] saving context <== verry intersting features
- [ ] select profile of workload
- [ ] add gpu support
- [ ] scale horizontaly
- [ ] manny user beter feature


## spec
- one part is used to parse a yaml containing login and settings
- one onther part is dedicated to spawn the worker and manage it life cycle 
- one part manage bot interaction with matrix and scheduling worker

it use massivly tokio capabilities and async arc and mutex lock and mscp  
