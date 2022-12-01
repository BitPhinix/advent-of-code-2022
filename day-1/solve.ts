const input = await Deno.readTextFile("./input.txt");

const elves = [{ calories: 0 }];
for (const line of input.split("\n")) {
  if (!line.length) {
    elves.push({ calories: 0 });
    continue;
  }

  elves[elves.length - 1].calories += parseInt(line, 10);
}

const max = elves.reduce((max, { calories }) => Math.max(max, calories), 0);
console.log(`part 1: ${max}`);

const top3 = elves.sort((a, b) => b.calories - a.calories).slice(0, 3);
const top3Sum = top3.reduce((total, { calories }) => total + calories, 0);
console.log(`part 2: ${top3Sum}`);
