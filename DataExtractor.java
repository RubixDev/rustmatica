/**
 * This file contains java code to get the block list from minecraft.
 * It is run by `run_data_extractor.bash` inside a temporary mod.
 * The output is piped into `blocks.txt`. After this you can run
 * make_lists.py to generate the Rust source files.
 */

package net.fabricmc.example;

import net.fabricmc.api.ModInitializer;
import net.minecraft.block.Block;
import net.minecraft.block.Blocks;
import net.minecraft.state.property.*;
import net.minecraft.util.StringIdentifiable;
import net.minecraft.util.registry.Registry;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.lang.reflect.Field;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class ExampleMod implements ModInitializer {
	public static final Logger LOGGER = LoggerFactory.getLogger("modid");

	@Override
	public void onInitialize() {
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

		System.exit(0);
	}
}
