const entities = get_characters(world);

print(entities);

const healths = get_health(world);

for health in healths {
    print(health);
}

const positions = get_positions(world);

for position in positions {
    print(position);
}
