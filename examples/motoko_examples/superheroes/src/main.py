from kybra import alias, nat32, opt, query, Record, update

SuperheroId = alias[nat32]

List = tuple[str, opt['List']]


# type of a superhero.
class Superhero(Record):
    name: str
    superpowers: opt[List]


##########################
### Application State ####
##########################

# The next available superhero identifier.
next: SuperheroId = 0

# The superhero data store.
superheroes: dict[SuperheroId, Superhero] = {}

##########################
### High-Level API #######
##########################


@update
# Create a superhero
def create(superhero: Superhero) -> SuperheroId:
    global next

    superhero_id = next
    next += 1
    superheroes[superhero_id] = superhero
    return superhero_id


@query
# Read a superhero
def read(superhero_id: SuperheroId) -> opt[Superhero]:
    return superheroes[superhero_id] if superhero_id in superheroes else None


@update
# Update a superhero
def update_(superhero_id: SuperheroId, superhero: Superhero) -> bool:
    result = superhero_id in superheroes
    if result:
        superheroes[superhero_id] = superhero
    return result


@update
# Delete a superhero
def delete_hero(superhero_id: SuperheroId) -> bool:
    result = superhero_id in superheroes
    if result:
        del superheroes[superhero_id]
    return result
