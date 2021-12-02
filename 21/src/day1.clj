(ns day1
  (:require [clojure.string :as str]))

(def input (map #(Integer/parseInt %) (str/split-lines (slurp (clojure.java.io/resource "1.txt")))))

(defn part1
  [input] (->> input
            (partition 2 1)
            (filter #(let [[a b] %] (< a b)))
            (count)))


(defn part2
  [input] (->> input
            (partition 3 1)
            (map #(apply + %))
            (partition 2 1)
            (filter #(let [[a b] %] (< a b)))
            (count)))

(defn -main
  [] (do
       (println "Part 1:" (part1 input))
       (println "Part 2:" (part2 input))))
