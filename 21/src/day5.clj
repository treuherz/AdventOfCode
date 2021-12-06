(ns day5
  (:require [clojure.string :as str])
  (:use [clojure.tools.trace :as trace]))

(defn parse-int [s] (Integer/parseInt s))

(defn parse [file]
  (->> file
    (clojure.java.io/resource)
    (slurp)
    (str/split-lines)
    (map #(rest (re-find #"(\d+),(\d+) -> (\d+),(\d+)" %)))
    (map #(map parse-int %))))

(defn not-diagonal? [[x1 y1 x2 y2]] (or (= x1 x2) (= y1 y2)))

(defn line [[x1 y1 x2 y2]]
  (let
    [x-step (if (< x2 x1) -1 1)
     y-step (if (< y2 y1) -1 1)]
    (cond
      (= x1 x2) (for [y (range y1 (+ y2 y-step) y-step)] [x1 y])
      (= y1 y2) (for [x (range x1 (+ x2 x-step) x-step)] [x y1])
      ;; We know other lines are 45°
      :else (map vector (range x1 (+ x2 x-step) x-step) (range y1 (+ y2 y-step) y-step)))))

(defn part1 [input]
  (->> input
    (filter not-diagonal?)
    (map line)
    (apply concat)
    (frequencies)
    (filter (fn [[_ count]] (>= count 2)))
    (count)))

(defn part2 [input]
  (->> input
    (map line)
    (apply concat)
    (frequencies)
    (filter (fn [[_ count]] (>= count 2)))
    (count)))

(defn -main []
  (let [input (trace (parse "5.txt"))]
    (println "Part 1:" (part1 input))
    (println "Part 2:" (part2 input))))
