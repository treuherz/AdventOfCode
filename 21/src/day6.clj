(ns day6
  (:require [clojure.string :as str])
  (:use [clojure.tools.trace :as trace]))

(defn parse [file]
  (as-> file v
    (clojure.java.io/resource v)
    (slurp v)
    (str/trim-newline v)
    (str/split v #",")
    (map #(Integer/parseInt %) v)))

(def timer-init 8)

(def timer-reset 6)

(def max-timer timer-init)

;; advance the state by decrementing each fish's age individually,
;; adding newborns and resetting timers as necessary
(defn advance [fish-list]
  (as-> fish-list v
    (for [f v] (- f 1))
    (concat v (repeat (count (filter #(= % -1) v)) timer-init))
    (for [f v] (if (= f -1) timer-reset f))))

(defn age [fish-map]
  (into (sorted-map)
    (for
      [t (range 0 max-timer) :let [t* (+ t 1)]]
      [t (get fish-map t* 0)])))

(defn advance* [fish-map]
  (merge-with +
    (age fish-map)
    {timer-init  (get fish-map 0 0)
     timer-reset (get fish-map 0 0)}))

(defn pop-after-n [input n] (count (nth (iterate advance input) n)))

(defn pop-after-n* [input n]
  (as-> input v
    (frequencies v)
    (iterate advance* v)
    (nth v n)
    (vals v)
    (reduce + v)))

(defn part1 [input] (pop-after-n input 80))

(defn part1* [input] (pop-after-n* input 80))

(defn part2 [input] (pop-after-n* input 256))

(defn -main []
  (let [input (parse "6.txt")]
    (println "Part 1 (impl 1):" (part1 input))
    (println "Part 1 (impl 2):" (part1* input))
    (println "Part 2:" (part2 input))))
