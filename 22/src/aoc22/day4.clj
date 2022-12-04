(ns aoc22.day4
  (:require
   [clojure.java.io :as io]
   [clojure.string :as str]
   [clojure.test :refer [deftest is]]))

(defn parse [s]
  (->> s
       str/split-lines
       (map #(str/split % #"[-,]"))
       (map #(map parse-long %))
       (map #(split-at 2 %))))

(defn either-contains [[a1 a2] [b1 b2]] (or (<= a1 b1 b2 a2) (<= b1 a1 a2 b2)))

(defn part1 [input]
  (->> input
       (filter #(apply either-contains %))
       count))

(defn overlaps [[a1 a2] [b1 b2]] (and (<= a1 b2) (<= b1 a2)))

(defn part2 [input]
  (->> input
       (filter #(apply overlaps %))
       count))

(deftest tests
  (let [sample "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8"
        input  (parse sample)]
    (is (= input [[[2 4] [6 8]] [[2 3] [4 5]] [[5 7] [7 9]] [[2 8] [3 7]] [[6 6] [4 6]] [[2 6] [4 8]]]) "parse")
    (is (= (part1 input) 2) "part1")
    (is (= (part2 input) 4) "part2")))

(defn -main []
  (let [file  (slurp (io/resource "4.txt"))
        input (parse file)]
    (println "Part 1:" (part1 input))
    (println "Part 2:" (part2 input))))
