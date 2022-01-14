from typing import TypeVar, TypedDict, Literal
from pathlib import Path
import json

import matplotlib.pyplot as plt


T = TypeVar("T")


def flatten(matrix: list[list[T]]) -> list[T]:
    return [cell for line in matrix for cell in line]


class Unit(TypedDict):
    id: int
    kind: Literal["Plant", "VegeterianAnimal", "PredatorAnimal"]
    quantity: float


class GroundCell(TypedDict):
    Ground: list[Unit]


Cell = GroundCell | Literal["Water"]


class CellStats(TypedDict):
    plants: float
    vegeterians: float
    predators: float


class CellStatSequence(TypedDict):
    plants: list[float]
    vegeterians: list[float]
    predators: list[float]


def cell_stats(cell: Cell) -> CellStats:
    plants = 0.0
    vegeterians = 0.0
    predators = 0.0
    match cell:
        case {"Ground": units}:
            for unit in units:
                match unit:
                    case {"id": _, "kind": "Plant", "quantity": quantity}:
                        plants += quantity
                    case {"id": _, "kind": "VegeterianAnimal", "quantity": quantity}:
                        vegeterians += quantity
                    case {"id": _, "kind": "PredatorAnimal", "quantity": quantity}:
                        predators += quantity
        case "Water":
            pass

    return {
        "plants": plants,
        "vegeterians": vegeterians,
        "predators": predators
    }


def fetch_stats(evolution: list[list[list[Cell]]]):
    result: list[CellStatSequence] = []
    for generation in evolution:
        cells = flatten(generation)
        for (idx, cell) in enumerate(cells):
            stats = cell_stats(cell)
            if len(result) < idx + 1:
                sequence: CellStatSequence = {
                    "plants": [stats["plants"]],
                    "vegeterians": [stats["vegeterians"]],
                    "predators": [stats["predators"]],
                }
                result.append(sequence)
            else:
                result[idx]["plants"].append(stats["plants"])
                result[idx]["vegeterians"].append(stats["vegeterians"])
                result[idx]["predators"].append(stats["predators"])
    return result


def print_cells(matrix: list[list[Cell]]):
    cells = flatten(matrix)
    for cell in cells:
        stats = cell_stats(cell)
        plants = stats["plants"]
        vegeterians = stats["vegeterians"]
        predators = stats["predators"]
        print(f"{plants=};{vegeterians=};{predators=}")


def experiment0(i18n):
    filename = "./experiment0.json"
    Path("experiment0").mkdir(parents=True, exist_ok=True)

    with open(filename) as f:
        data = json.load(f)
        evolution = fetch_stats(data)
        for (i, cell) in enumerate(evolution):
            plants = cell["plants"]
            vegeterians = cell["vegeterians"]
            predators = cell["predators"]
            t = range(len(plants))

            _, ax = plt.subplots()
            ax.plot(t, plants, 'g-', label=i18n["ua"]["plants"])
            ax.plot(t, vegeterians, 'y-', label=i18n["ua"]["vegeterians"])
            ax.plot(t, predators, 'r-', label=i18n["ua"]["predators"])

            ax.legend()
            plt.savefig(f"experiment0/{i}.png")


def main():
    i18n = {
        "ua": {
            "plants": "Рослини",
            "vegeterians": "Вегетеріанці",
            "predators": "Хижаки",
        },
    }
    experiment0(i18n)


if __name__ == "__main__":
    main()
