(ns day1
  (:require [clojure.string :as str]
            [clojure.test
             :refer
             [deftest is]])
  (:use [clojure.tools.trace]))

(defn split-linegroups [s] (str/split s #"\r?\n\r?\n"))

(defn parse [file]
  (->> file
       (split-linegroups)
       (map str/split-lines)
       (map (partial map parse-long))))

(defn sum-group [input] (map (partial reduce +) input))

(defn part1 [input] (apply max (sum-group input)))

(defn part2 [input] (reduce + (take 3 (sort > (sum-group input)))))

(deftest tests
  (let [sample "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000"
        input (parse sample)]
    (is (= input [[1000 2000 3000] [4000] [5000 6000] [7000 8000 9000] [10000]]) "parse")
    (is (= (part1 input) 24000) "part1")
    (is (= (part2 input) 45000) "part2")))

(defn -main []
  (let [file (slurp (clojure.java.io/resource "1.txt"))
        input (parse file)]
    (println "Part 1:" (part1 input))
    (println "Part 2:" (part2 input))))
