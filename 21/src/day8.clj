(ns day8
  (:require [clojure.string :as str])
  (:require [clojure.set :as set])
  (:use [clojure.tools.trace]))

(defn parse-line [line]
  (as-> line v
    (str/split v #" \| " 2)
    (mapv #(str/split % #"\s") v)
    (zipmap [:samples :output] v)))

(defn parse [file]
  (->> file
    (clojure.java.io/resource)
    (slurp)
    (str/split-lines)
    (map parse-line)))

(def segs
  {0 "abcefg"
   1 "cf"
   2 "acdeg"
   3 "acdfg"
   4 "bcdf"
   5 "abdfg"
   6 "abdefg"
   7 "acf"
   8 "abcdefg"
   9 "abcdfg"})

(def seg-count
  (into {} (for [[k v] segs] [k (count v)])))

(defn part1 [input]
  (->> input
    (map :output)
    (flatten)
    (map count)
    (filter (partial contains?
              (into #{}
                (vals (select-keys seg-count #{1 4 7 8})))))
    (count)))

(defn seek-and-assoc [n f [identified unidentified]]
  (let [s (first (filter (f identified) unidentified))] [(assoc identified n s) (disj unidentified s)]))

(defn identify-samples [samples]
  (->> [(sorted-map) (set (map set samples))]
    ;; This doesn't feel very idiomatic but works. We call seek-and-assoc, starting with all the
    ;; samples (as sets) and an empty map, and we use a function to find each digit in the samples
    ;; one by one, remove it from the set and add it to the map.
    ;; The order matters here, as some of these only find a unique sample if others have already been removed
    ;;
    ;; 1, 7, 4 and 9 have unique numbers of segments
    (seek-and-assoc 1 (constantly #(= 2 (count %))))
    (seek-and-assoc 7 (constantly #(= 3 (count %))))
    (seek-and-assoc 4 (constantly #(= 4 (count %))))
    (seek-and-assoc 8 (constantly #(= 7 (count %))))
    ;; 3 is the only 5-segment digit with its right two bars (i.e. "1") lit
    (seek-and-assoc 3 (fn [found] #(and (= 5 (count %)) (set/subset? (found 1) %))))
    ;; 9 is the only 6-segment digit all the segments of 3 lit
    (seek-and-assoc 9 (fn [found] #(and (= 6 (count %)) (set/subset? (found 3) %))))
    ;; 0 is the only remaining 6-segment digit all the segments of 1 lit
    (seek-and-assoc 0 (fn [found] #(and (= 6 (count %)) (set/subset? (found 1) %))))
    ;; 6 is the only remaining 6-segment digit
    (seek-and-assoc 6 (constantly #(= 6 (count %))))
    ;; 5 is the only remaining 5-segment digit with no segments lit that weren't in 9
    (seek-and-assoc 5 (fn [found] #(and (= 5 (count %)) (set/superset? (found 9) %))))
    ;; 5 is the only remaining digit
    (seek-and-assoc 2 (constantly any?))
    ;; Turn the sets back into strings
    (first)
    (reduce-kv #(assoc %1 %2 (apply str (sort %3))) {})
    (set/map-invert)))

(defn identify-digit [digit lookup] (lookup (apply str (sort digit))))

(defn identify [{:keys [samples output]}]
  (let [lookup (identify-samples samples)
        digits (for [digit output] (identify-digit digit lookup))]
    (reduce #(+ (* 10 %1) %2) digits)))

(defn part2 [input] (reduce + (map identify input)))

(defn -main []
  (let [input (trace (parse "8.txt"))]
    (println "Part 1:" (part1 input))
    (println "Part 2:" (part2 input))))
