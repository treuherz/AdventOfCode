(ns day7
  (:require [clojure.string :as str])
  (:use [clojure.tools.trace :as trace]))

(defn parse-int [s] (Integer/parseInt s))

(defn parse [file]
  (as-> file v
    (clojure.java.io/resource v)
    (slurp v)
    (str/trim-newline v)
    (str/split v #",")
    (map #(Integer/parseInt %) v)))

(defn fuel [positions target]
  (->> positions
    (map #(Math/abs ^int (- % target)))
    (reduce +)))

(defn fuel* [positions target]
  (->> positions
    (map #(Math/abs ^int (- % target)))
    (map #(/ (* % (+ % 1)) 2))
    (reduce +)))

(defn part1 [input]
  (->> (for [t (range 0 (apply max input))] (fuel input t))
    (map-indexed vector)
    (apply min-key second)
    (first)
    (fuel input)))

(defn part2 [input]
  (->> (for [t (range 0 (apply max input))] (fuel* input t))
    (map-indexed vector)
    (apply min-key second)
    (first)
    (fuel* input)))

(defn -main []
  (let [input (trace (parse "7.txt"))]
    (println "Part 1:" (part1 input))
    (println "Part 2:" (part2 input))))
