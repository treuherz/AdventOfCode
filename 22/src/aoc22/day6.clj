(ns aoc22.day6
  (:require
   [clojure.java.io :as io]
   [clojure.test :refer [deftest is]]))

(defn index-after-first-distinct-window [input length]
  (->> input
       (partition length 1)
       (map (comp count distinct))
       (keep-indexed #(when (= length %2) %1))
       first
       (+ length)))

(defn part1 [input] (index-after-first-distinct-window input 4))

(defn part2 [input] (index-after-first-distinct-window input 14))

(deftest tests
  (let [sample "mjqjpqmgbljsphdztnvjfqwrcgsmlb"
        input  sample]
    (is (= (part1 input) 7) "part1")
    (is (= (part2 input) 19) "part2")))

(defn -main []
  (let [file  (slurp (io/resource "6.txt"))
        input file]
    (println "Part 1:" (part1 input))
    (println "Part 2:" (part2 input))))
