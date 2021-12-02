(ns day1
  (:require [clojure.string :as str]))

(def input (map #(Integer/parseInt %) (str/split-lines (slurp (clojure.java.io/resource "1.txt")))))

(defn part1
  [input] (let [pairs (partition 2 1 input)]
            (count (filter #(let [[a b] %] (< a b)) pairs))))

(defn part2
  [input] (let [runs (partition 3 1 input)
                sums (map #(apply + %) runs)
                pairs (partition 2 1 sums)]
            (count (filter #(let [[a b] %] (< a b)) pairs))))

(defn -main
  [] (do
       (println "Part 1:" (part1 input))
       (println "Part 2:" (part2 input))))
