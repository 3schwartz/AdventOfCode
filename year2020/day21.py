import common

lines = common.get_lines('day21_data.txt')

food_dict = {}
all_ingredients = []
for line in lines:
    ingredients, allergens = line.split(' (contains ')
    allergens = allergens[:-1].split(', ')
    ingredients = ingredients.split(' ')

    all_ingredients.extend(ingredients)

    for allergen in allergens:
        if food_dict.get(allergen) is None:
            food_dict[allergen] = set(ingredients)
        else:
            food_dict[allergen] = food_dict[allergen].intersection(set(ingredients))

safe_ingredients = set(all_ingredients) \
    .difference(set(ingredient
                    for value in food_dict.values()
                    for ingredient in value))

print(f"Part 1: {sum(ingredient in safe_ingredients for ingredient in all_ingredients)}")

while any(len(ingredients) > 1 for ingredients in food_dict.values()):
    for allergen, ingredients in food_dict.items():
        if len(ingredients) == 1:
            for inner_allergen, inner_ingredients in food_dict.items():
                if allergen == inner_allergen:
                    continue

                ingredient = next(iter(ingredients))

                if ingredient in inner_ingredients:
                    food_dict[inner_allergen].remove(ingredient)

names = list(food_dict.keys())
names.sort()

print(f"Part 2: {','.join(next(iter(food_dict[allergen])) for allergen in names)}")

