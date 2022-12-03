(ns aoc22.day3
  (:require
   [clojure.java.io :as io]
   [clojure.set :as set]
   [clojure.string :as str]
   [clojure.test :refer [deftest is]]))

(defn parse [s]
  (->> s
       str/split-lines
       (map #(split-at (/ (count %) 2) %))
       (map #(map char-array %))
       (map #(map set %))))

(defn priority [c]
  (if
   (<= (int \a) c (int \z))
    (- c 96)
    (- c 38)))

(defn part1 [input]
  (->> input
       (map (fn [[a b]] (set/intersection a b)))
       (map first)
       (map int)
       (map priority)
       (reduce +)))

(defn part2 [input]
  (->> input
       (map #(apply set/union %))
       (partition 3)
       (map flatten)
       (map #(apply set/intersection %))
       (map first)
       (map int)
       (map priority)
       (reduce +)))

(deftest tests
  (let [sample "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw"
        input  (parse sample)]
    (is (= (first input) [#{\v \J \r \w \p \W \t \g} #{\h \c \s \F \M \f \p}]) "parse")
    (is (= (part1 input) 157) "part1")
    (is (= (part2 input) 70) "part2")))

(defn -main []
  (let [file  (slurp (io/resource "3.txt"))
        input (parse file)]
    (println "Part 1:" (part1 input))
    (println "Part 2:" (part2 input))))
