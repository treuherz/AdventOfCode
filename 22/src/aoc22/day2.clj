(ns aoc22.day2
  (:require
   [clojure.java.io :as io]
   [clojure.string :as str]
   [clojure.test :refer [deftest is]]))

(defn first-as-char [s] (first (char-array s)))

(defn parse [s]
  (->> s
       str/split-lines
       (map #(str/split % #" "))
       (map #(map first-as-char %))))

(defn char->score [c]
  (case c
    \A 1
    \X 1
    \B 2
    \Y 2
    \C 3
    \Z 3))

(defn part1 [input]
  (reduce + (map
             (fn [[them me]]
               (+
                (char->score me)
                (case [them me]
                  [\A \X] 3
                  [\A \Y] 6
                  [\A \Z] 0
                  [\B \X] 0
                  [\B \Y] 3
                  [\B \Z] 6
                  [\C \X] 6
                  [\C \Y] 0
                  [\C \Z] 3)))
             input)))

(defn part2 [input]
  (reduce + (map
             (fn [[them outcome]]
               (+
                (condp = outcome
                  \X 0
                  \Y 3
                  \Z 6)
                (condp = [outcome them]
                  [\X \A] 3
                  [\X \B] 1
                  [\X \C] 2
                  [\Y \A] 1
                  [\Y \B] 2
                  [\Y \C] 3
                  [\Z \A] 2
                  [\Z \B] 3
                  [\Z \C] 1)))
             input)))

(deftest tests
  (let [sample "A Y\nB X\nC Z"
        input  (parse sample)]
    (is (= input [[\A \Y] [\B \X] [\C \Z]]) "parse")
    (is (= (part1 input) 15) "part1")
    (is (= (part2 input) 12) "part2")))

(defn -main []
  (let [file  (slurp (io/resource "2.txt"))
        input (parse file)]
    (println "Part 1:" (part1 input))
    (println "Part 2:" (part2 input))))
