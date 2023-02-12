## Logs
Debug and trace logs may contain secrets and are to be treated as if they do

## INIT
you nay need to connect to docker via
`docker exec -it cockroach1 bash`
and then init cockroach db via `cocroach init --insecure`