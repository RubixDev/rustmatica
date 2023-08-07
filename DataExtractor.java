/**
 * This file contains java code to get block states and other info
 * from minecraft. It is run by `run_data_extractor.sh` inside a
 * temporary mod. The output is piped into `data.txt`. After this
 * you can run make_lists.py to generate the Rust source files.
 */
package com.example;

import net.fabricmc.api.EnvType;
import net.fabricmc.api.Environment;
import net.fabricmc.api.ModInitializer;
import net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents;
import net.minecraft.block.Block;
import net.minecraft.block.Blocks;
import net.minecraft.entity.Entity;
import net.minecraft.entity.EntityType;
import net.minecraft.registry.Registries;
import net.minecraft.server.MinecraftServer;
import net.minecraft.server.world.ServerWorld;
import net.minecraft.state.property.*;
import net.minecraft.util.StringIdentifiable;
import net.minecraft.world.World;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.lang.reflect.Field;
import java.lang.reflect.Modifier;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;

@Environment(EnvType.SERVER)
public class ExampleMod implements ModInitializer {
    public static final Logger LOGGER = LoggerFactory.getLogger("modid");

    @Override
    public void onInitialize() {
        // ! get block info
        Map<String, List<String>> enums = new HashMap<>();

        for (Field field : Blocks.class.getDeclaredFields()) {
            try {
                Block block = (Block) field.get(null);
                String name = Registries.BLOCK.getId(block).toString().replaceAll("minecraft:", "");
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
                        List<String> enumValues = property.getValues().stream()
                                .map(value -> {
                                    if (value instanceof StringIdentifiable)
                                        return ((StringIdentifiable) value).asString();
                                    return value.toString();
                                })
                                .toList();
                        if (enums.containsKey(type) && !enums.get(type).equals(enumValues)) {
                            LOGGER.error("Ambiguous enum type for: " + name + " " + type + " -- " + enumValues + " -- "
                                    + enums.get(type));
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

        // ! get enum info
        System.err.print("\n");

        for (Map.Entry<String, List<String>> set : enums.entrySet()) {
            System.err.print("ENUMINFO --- " + set.getKey() + " - " + String.join(",", set.getValue()) + "\n");
        }

        // other work is yet to be done
        ServerLifecycleEvents.SERVER_STARTED.register(new ServerStartHandler());
    }
}

class ServerStartHandler implements ServerLifecycleEvents.ServerStarted {
    @Override
    public void onServerStarted(MinecraftServer server) {
        // ! get entity info
        System.err.print("\n");

        // create a dummy world in order to spawn entities from entitytypes
        ServerWorld world = server.getWorld(World.OVERWORLD);
        List<Class<?>> allEntityClasses = new ArrayList<>();

        for (Field field : EntityType.class.getDeclaredFields()) {
            try {
                if (!Modifier.isStatic(field.getModifiers())) continue;

                EntityType<?> entityType = (EntityType<?>) field.get(null);
                if (entityType == EntityType.PLAYER) continue;

                System.err.print("ENTITYINFO --- " + EntityType.getId(entityType));

                Entity entity = entityType.create(world);
                Class<?> entityClass;

                if (entity == null) {
                    System.err.print("\n");
                    ExampleMod.LOGGER.error(EntityType.getId(entityType).toString());
                    continue;
                } else {
                    entityClass = entity.getClass();
                }

                System.err.print(" - " + classFile(entityClass) + "\n");
                allEntityClasses.add(entityClass);

                for (Class<?> superClass : getAllSuperClasses(entityClass)) {
                    if (allEntityClasses.contains(superClass)) continue;
                    allEntityClasses.add(superClass);
                }
            } catch (IllegalAccessException | ClassCastException ignored) {
            }
        }

        // ! get entity class info
        System.err.print("\n");
        for (Class<?> entityClass : allEntityClasses) {
            System.err.print("ENTITYCLASSINFO --- " + classFile(entityClass) + " - "
                    + getAllSuperClasses(entityClass).stream()
                    .map(this::classFile)
                    .collect(Collectors.joining(",")) + "\n");
        }

        server.stop(false);
    }

    private List<Class<?>> getAllSuperClasses(Class<?> clazz) {
        List<Class<?>> superClasses = new ArrayList<>(List.of(clazz.getInterfaces()));
        Class<?> superClass = clazz.getSuperclass();
        while (superClass != null && superClass != Object.class) {
            superClasses.add(superClass);
            superClasses.addAll(List.of(superClass.getInterfaces()));
            superClass = superClass.getSuperclass();
        }
        return superClasses;
    }

    private String classFile(Class<?> clazz) {
        return clazz.getName().replace('.', '/') + ".java";
    }
}
