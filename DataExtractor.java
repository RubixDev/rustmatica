/**
 * This file contains java code to get the block list from minecraft.
 * It is run by `run_data_extractor.bash` inside a temporary mod.
 * The output is piped into `data.txt`. After this you can run
 * make_lists.py to generate the Rust source files.
 */

package net.fabricmc.example;

import com.mojang.serialization.Lifecycle;
import net.fabricmc.api.ModInitializer;
import net.minecraft.block.Block;
import net.minecraft.block.BlockState;
import net.minecraft.block.Blocks;
import net.minecraft.entity.Entity;
import net.minecraft.entity.EntityType;
import net.minecraft.entity.player.PlayerEntity;
import net.minecraft.fluid.Fluid;
import net.minecraft.item.map.MapState;
import net.minecraft.recipe.RecipeManager;
import net.minecraft.resource.DataPackSettings;
import net.minecraft.scoreboard.Scoreboard;
import net.minecraft.sound.SoundCategory;
import net.minecraft.sound.SoundEvent;
import net.minecraft.state.property.Properties;
import net.minecraft.state.property.*;
import net.minecraft.tag.BlockTags;
import net.minecraft.util.StringIdentifiable;
import net.minecraft.util.math.BlockPos;
import net.minecraft.util.math.Direction;
import net.minecraft.util.registry.DynamicRegistryManager;
import net.minecraft.util.registry.Registry;
import net.minecraft.util.registry.RegistryEntry;
import net.minecraft.world.Difficulty;
import net.minecraft.world.GameMode;
import net.minecraft.world.GameRules;
import net.minecraft.world.World;
import net.minecraft.world.biome.Biome;
import net.minecraft.world.chunk.ChunkManager;
import net.minecraft.world.dimension.DimensionType;
import net.minecraft.world.entity.EntityLookup;
import net.minecraft.world.event.GameEvent;
import net.minecraft.world.gen.GeneratorOptions;
import net.minecraft.world.level.LevelInfo;
import net.minecraft.world.level.LevelProperties;
import net.minecraft.world.tick.QueryableTickScheduler;
import org.jetbrains.annotations.Nullable;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.lang.reflect.Field;
import java.lang.reflect.Modifier;
import java.util.*;
import java.util.stream.Collectors;

public class ExampleMod implements ModInitializer {
	public static final Logger LOGGER = LoggerFactory.getLogger("modid");

	@Override
	public void onInitialize() {
//        List<Block> allBlocks = new ArrayList<>();
		Map<String, List<String>> enums = new HashMap<>();
		for (Field field : Blocks.class.getDeclaredFields()) {
			try {
				Block block = (Block) field.get(null);
//                allBlocks.add(block);
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
							LOGGER.error("Ambiguous enum type for: " + name + " " + type + " -- " + enumValues + " -- " + enums.get(type));
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

		/* Useful when I am somehow able to extract the properties per tile entity */
//         System.err.print("\n");
//         for (Field field : BlockEntityType.class.getDeclaredFields()) {
//             try {
//                 if (!Modifier.isStatic(field.getModifiers())) continue;
//                 BlockEntityType<?> entityType = (BlockEntityType<?>) field.get(null);
//                 System.err.print("TILEENTITYINFO --- "+ BlockEntityType.getId(entityType));
//                 List<Block> supported = allBlocks.stream().filter(block -> entityType.supports(block.getDefaultState())).toList();
//                 System.err.print(
//                         " - " + supported.stream()
//                                 .map(block -> Registry.BLOCK.getId(block).toString().replaceAll("minecraft:", ""))
//                                 .collect(Collectors.joining(",")) + "\n"
//                 );
// //				List<BlockEntity> entities = supported.stream()
// //						.map(block -> (BlockEntity) entityType.instantiate(BlockPos.ORIGIN, block.getDefaultState()))
// //						.filter(Objects::nonNull)
// //						.toList();
// //				if (entities.size() == 0) {
// //					LOGGER.error(supported.get(0).toString());
// //					continue;
// //				}
// //				BlockEntity entity = entities.get(0);
// //				System.err.print(" -- " + entity.getClass().getName().replace('.', '/') + ".java\n");
//             } catch (IllegalAccessException ignored) {
//             }
//         }

		System.err.print("\n");
		// Create a dummy world in order to spawn Entities from EntityTypes
		World world = new DummyWorld();
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
					LOGGER.error(EntityType.getId(entityType).toString());
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

		System.err.print("\n");
		for (Class<?> entityClass : allEntityClasses) {
			System.err.print(
					"ENTITYCLASSINFO --- " + classFile(entityClass) + " - "
							+ getAllSuperClasses(entityClass).stream().map(this::classFile).collect(Collectors.joining(","))
							+ "\n"
			);
		}

		System.exit(0);
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

class DummyWorld extends World {
	public DummyWorld() {
		super(
				new LevelProperties(
						new LevelInfo("dummy", GameMode.CREATIVE, false, Difficulty.HARD,
								true, new GameRules(),
								new DataPackSettings(new ArrayList<>(), new ArrayList<>())),
						GeneratorOptions.getDefaultOptions(DynamicRegistryManager.createAndLoad()),
						Lifecycle.stable()
				), null,
				RegistryEntry.of(DimensionType.create(OptionalLong.empty(),
						true, false, false, true, 1.0,
						false, false, true, false,
						true, -64, 384, 384, BlockTags.INFINIBURN_OVERWORLD,
						DimensionType.OVERWORLD_ID, 0.0f)
				), null, false, false, 1
		);
	}

	@Override public void updateListeners(BlockPos pos, BlockState oldState, BlockState newState, int flags) {}
	@Override public void playSound(@Nullable PlayerEntity except, double x, double y, double z, SoundEvent sound, SoundCategory category, float volume, float pitch) {}
	@Override public void playSoundFromEntity(@Nullable PlayerEntity except, Entity entity, SoundEvent sound, SoundCategory category, float volume, float pitch) {}
	@Override public String asString() { return null; }
	@Nullable @Override public Entity getEntityById(int id) { return null; }
	@Nullable @Override public MapState getMapState(String id) { return null; }
	@Override public void putMapState(String id, MapState state) {}
	@Override public int getNextMapId() { return 0; }
	@Override public void setBlockBreakingInfo(int entityId, BlockPos pos, int progress) {}
	@Override public Scoreboard getScoreboard() { return new Scoreboard(); }
	@Override public RecipeManager getRecipeManager() { return null; }
	@Override protected EntityLookup<Entity> getEntityLookup() { return null; }
	@Override public QueryableTickScheduler<Block> getBlockTickScheduler() { return null; }
	@Override public QueryableTickScheduler<Fluid> getFluidTickScheduler() { return null; }
	@Override public ChunkManager getChunkManager() { return null; }
	@Override public void syncWorldEvent(@Nullable PlayerEntity player, int eventId, BlockPos pos, int data) {}
	@Override public void emitGameEvent(@Nullable Entity entity, GameEvent event, BlockPos pos) {}
	@Override public DynamicRegistryManager getRegistryManager() { return null; }
	@Override public float getBrightness(Direction direction, boolean shaded) { return 0; }
	@Override public List<? extends PlayerEntity> getPlayers() { return null; }
	@Override public RegistryEntry<Biome> getGeneratorStoredBiome(int biomeX, int biomeY, int biomeZ) { return null; }
}
