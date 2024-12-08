#include <iostream>
#include <vector>
#include <variant>
#include <optional>
#include <algorithm>
#include <string>
#include <fstream>

enum Directions {
    North,
    South,
    East,
    West
};

class MapField {
    public:
        MapField(const char& field_type): field_type_(field_type), visited_() {}
        MapField(const MapField& obj): field_type_(obj.field_type_), visited_(obj.visited_) {}
        char field_type_;
        std::vector<Directions> visited_;
    private:
        
};

struct Finished {
    std::size_t count;
    std::vector<std::pair<std::size_t, std::size_t> > visited;
};

using MapResult = std::variant<std::monostate, Finished>;

std::optional<std::pair<std::size_t, std::size_t>> get_start_pos(std::vector<std::vector<MapField>>& map) {
    for (std::size_t i = 0; i < map.size(); i++) {
        for (std::size_t j = 0; j < map[i].size(); j++) {
            if (map[i][j].field_type_ == '^') {
                return std::pair(i, j);
            }
        }
    }
    return (std::nullopt);
}

MapResult run_simulation(std::vector<std::vector<MapField>>& og_map) {
    std::vector<std::vector<MapField>> map = og_map;
    std::pair<std::size_t, std::size_t> startpositions = get_start_pos(map).value();
    Directions direction = North;

    std::size_t y = startpositions.first;
    std::size_t x = startpositions.second;
    std::size_t visited_count = 0;
    std::vector<std::pair<std::size_t, std::size_t>> visited_fields;

    while (true) {
        auto it = std::find(map[y][x].visited_.begin(), map[y][x].visited_.end(), direction);

        if (it != map[y][x].visited_.end()) {
            return (std::monostate());
        }

        if (map[y][x].visited_.empty()) {
            visited_count++;
            if (y != startpositions.first || x != startpositions.second) {
                visited_fields.push_back(std::pair(y, x));
            }
        }

        map[y][x].visited_.push_back(direction);

        if (y == 0 || x == 0 || y == map.size() - 1 || x == map[y].size() - 1) {
            Finished result;
            result.count = visited_count;
            result.visited = visited_fields;
            return (result);
        }

        switch (direction) {
            case North:
                if (map[y - 1][x].field_type_ == '#') {
                    direction = East;
                } else {
                    y -= 1;
                }
                break;
            case East:
                if (map[y][x + 1].field_type_ == '#') {
                    direction = South;
                } else {
                    x += 1;
                }
                break;
            case South:
                if (map[y + 1][x].field_type_ == '#') {
                    direction = West;
                } else {
                    y += 1;
                }
                break;
            default:
                if (map[y][x - 1].field_type_ == '#') {
                    direction = North;
                } else {
                    x -= 1;
                }
        }
    }
    return (std::monostate());
}

std::vector<std::vector<MapField>> convert_input(std::vector<std::string>& lines) {
    std::vector<std::vector<MapField>> mapfields;
    for (std::size_t i = 0; i < lines.size(); i++) {
        std::vector<MapField> fields_in_line;
        for (std::size_t j = 0; j < lines[i].size(); j++) {
            fields_in_line.push_back(MapField(lines[i][j]));
        }
        mapfields.push_back(fields_in_line);
    }
    return (mapfields);
}

int main(void) {
    std::ifstream file("input.txt");

    std::vector<std::string> lines;
    std::string line;

    while(std::getline(file, line)) {
        lines.push_back(line);
    }
    file.close();

    std::vector<std::vector<MapField>> map = convert_input(lines);
    MapResult result = run_simulation(map);

    if (std::holds_alternative<Finished>(result)) {
        Finished resultitem = std::get<Finished>(result);
        std::cout << "Part 1: " << resultitem.count << std::endl;
        std::size_t infinite_count = 0;
        for (const std::pair<std::size_t, std::size_t>& visitedfield: resultitem.visited) {
            map[visitedfield.first][visitedfield.second] = '#';
            if (std::holds_alternative<std::monostate>(run_simulation(map))) {
                infinite_count++;
            }
            map[visitedfield.first][visitedfield.second] = '.';
        }
        std::cout << "Part 2: " << infinite_count << std::endl;
    }
}