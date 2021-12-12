(ns day11
  (:require [clojure.string :as str])
  (:require [clojure.set :as set])
  (:use [clojure.tools.trace]))

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

(defn neighbour-coords [[y x]]
  [[y (dec x)]
   [y (inc x)]
   [(dec y) x]
   [(inc y) x]
   [(dec y) (dec x)]
   [(dec y) (inc x)]
   [(inc y) (dec x)]
   [(inc y) (inc x)]])

(defn all-coords [grid]
  (for [y (range 0 (count grid))
        x (range 0 (count (first (vals grid))))]
    [y x]))

(defn update-all [grid f]
  (reduce (fn [grid coord]
            (do
              (when (not (get-in grid coord))
                (trace [grid coord]))
              (update-in grid coord f)))
    grid
    (all-coords grid)))

(defn inc-all [grid] (update-all grid inc))

(defn reset-flashed [grid] (update-all grid #(if (< 9 %) 0 %)))

(defn flash-at [grid [y x]]
  (reduce
    (fn [grid coord] (update-in grid coord #(when % (inc %))))
    grid
    ;; neighbours, filtering out nils
    (filter #(get-in grid %) (neighbour-coords [y x]))))

(defn turn [grid]
  (loop [flashed #{}
         grid (inc-all grid)]
    (let [could-flash (filter #(< 9 (get-in grid %)) (all-coords grid))
          will-flash (set/difference (set could-flash) flashed)]
      (if (empty? will-flash)
        [(reset-flashed grid) (count flashed)]
        (recur
          (set/union flashed will-flash)
          (reduce flash-at grid will-flash))))))

(defn iterate-turn [grid]
  (iterate
    (fn [[grid cnt]]
      (let [[new-grid flashed] (turn grid)]
        [new-grid (+ cnt flashed)]))
    [grid 0]))

(defn part1 [input] (second (nth (iterate-turn input) 100)))

(defn part2 [input]
  (let [size (* (count input) (count (first (vals input))))]
    (loop [n 1
           grid input]
      (let [[new-grid flashed] (turn grid)]
        (if (=  flashed size)
          n
          (recur (inc n) new-grid))))))

(defn -main []
  (let [input (trace (parse "11.txt"))]
    (println "Part 1:" (part1 input))
    (println "Part 2:" (part2 input))))
