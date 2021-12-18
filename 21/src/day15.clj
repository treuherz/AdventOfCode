(ns day15
  (:require [clojure.string :as str])
  (:use [clojure.tools.trace])
  (:require [clojure.math.combinatorics :as combo]))

(defn parse [file]
  (->> file
    (clojure.java.io/resource)
    (slurp)
    (str/split-lines)
    (map (fn [line]
           (into (sorted-map)
             (map-indexed
               #(vector %1 (Character/digit ^char %2 10))
               line))))
    (map-indexed vector)
    (into (sorted-map))))

(defn part1 [input]
  (let [size           (count input)
        all-directions (combo/permutations (concat (repeat (dec size) :x) (repeat (dec size) :y)))]
    (->> all-directions
      (pmap (fn [dirs]
             (reduce
               (fn [{[y x] :pos, risk :risk} dir]
                 (let [pos* (case dir :x [y (inc x)], :y [(inc y) x])]
                   {:pos pos*, :risk (+ risk (get-in input pos*))}))
               {:pos [0 0], :risk 0}
               dirs)))
      (map :risk)
      (apply min))))

(defn part2 [input])

(defn -main []
  (let [input (trace (parse "15_sample.txt"))]
    (println "Part 1:" (part1 input))
    (println "Part 2:" (part2 input))))
