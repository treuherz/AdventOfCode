(ns day10
  (:require [clojure.string :as str])
  (:require [clojure.set :as set])
  (:use [clojure.tools.trace]))

(defn parse [file]
  (->> file
    (clojure.java.io/resource)
    (slurp)
    (str/split-lines)))

(defn stack-or-error [line]
  (reduce (fn [stack char]
            (case char
              \( (conj stack :paren)
              \[ (conj stack :square)
              \{ (conj stack :brace)
              \< (conj stack :angle)
              \) (if (= (peek stack) :paren) (pop stack) (reduced char))
              \] (if (= (peek stack) :square) (pop stack) (reduced char))
              \} (if (= (peek stack) :brace) (pop stack) (reduced char))
              \> (if (= (peek stack) :angle) (pop stack) (reduced char))
              ))
    (list) line))

(defn part1 [input]
  (->> input
    (map stack-or-error)
    ;; Find where we bailed out with a character
    (filter char?)
    (map #(case % \) 3, \] 57, \} 1197, \> 25137))
    (reduce +)))

(defn middle-value [coll]
  (let [sorted (sort coll)
        cnt    (count sorted)
        half   (quot cnt 2)]
    ;; ignore even-length coll
    (nth sorted half)))

(defn part2 [input]
  (->> input
    (map stack-or-error)
    ;; Find where we didn't bail out with a character
    (filter (complement char?))
    (map (fn [stack]
           (reduce (fn [score v]
                     (+ (* score 5)
                       (case v
                         :paren 1
                         :square 2
                         :brace 3
                         :angle 4)))
             0 stack)))
    (middle-value)))

(defn -main []
  (let [input (trace (parse "10.txt"))]
    (println "Part 1:" (part1 input))
    (println "Part 2:" (part2 input))))
