/**
 * This file contains java code to get the block list from minecraft.
 * To use, copy this code in a FabricMC mod with yarn mappings so that it
 * is run on startup. The debug info is printed to stderr, so to capture it
 * use a command like `./gradlew runServer 2> blocks.txt`.
 *
 * With that info placed in a file called `blocks.txt` you can run
 * make_blocks.py to generate the Rust source files.
 */

Map<String, List<String>> enums = new HashMap<>();
for (Field field : Blocks.class.getDeclaredFields()) {
    try {
        Block block = (Block) field.get(null);
        String name = Registry.BLOCK.getId(block).toString().replaceAll("minecraft:", "");
        System.err.print("BLOCKINFO --- " + name + " - ");
        for (Property<?> property : block.getDefaultState().getProperties()) {
            String type;
            if (property instanceof IntProperty) {
                type = "u8";
            } else if (property instanceof BooleanProperty) {
                type = "bool";
            } else if (property instanceof EnumProperty<?>) {
                if (property == Properties.HORIZONTAL_AXIS) {
                    type = "HorizontalAxis";
                } else if (property == Properties.HOPPER_FACING) {
                    type = "HopperDirection";
                } else if (property == Properties.HORIZONTAL_FACING) {
                    type = "HorizontalDirection";
                } else if (property == Properties.VERTICAL_DIRECTION) {
                    type = "VerticalDirection";
                } else if (property == Properties.STRAIGHT_RAIL_SHAPE) {
                    type = "StraightRailShape";
                } else {
                    type = property.getType().getSimpleName();
                }
                List<String> enumValues = property.getValues().stream().map(value -> {
                    if (value instanceof StringIdentifiable)
                        return ((StringIdentifiable) value).asString();
                    return value.toString();
                }).toList();
                if (enums.containsKey(type) && !enums.get(type).equals(enumValues)) {
                    System.out.println("Error: ambiguous enum type for: " + name + " " + type + " -- " + enumValues + " -- " + enums.get(type));
                } else {
                    enums.put(type, enumValues);
                }
            } else {
                type = "TODO";
            }
            System.err.print(property.getName() + ":" + type + " ");
        }
        System.err.println();
    } catch (IllegalAccessException ignored) {
    }
}

System.err.print("\n");
for (Map.Entry<String, List<String>> set : enums.entrySet()) {
    System.err.print("ENUMINFO --- " + set.getKey() + " - " + String.join(",", set.getValue()) + "\n");
}
