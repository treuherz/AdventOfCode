(ns day9
  (:require [clojure.string :as str])
  (:require [clojure.set :as set])
  (:use [clojure.tools.trace]))

(defn parse [file]
  (->> file
    (clojure.java.io/resource)
    (slurp)
    (str/split-lines)
    (map (fn [line] (into {}
                      (map-indexed
                        #(vector %1 (Character/digit ^char %2 10))
                        line))))
    (map-indexed vector)
    (into {})))

(defn neighbour-coords [[y x]]
  [[y (dec x)]
   [y (inc x)]
   [(dec y) x]
   [(inc y) x]])

(defn points-neighbours [input]
  (for [y (range 0 (count input))
        x (range 0 (count (first (vals input))))]
    {:point      (get-in input [y x])
     :coords     [y x]
     :neighbours (filter #(not (nil? %))
                   (for [coords (neighbour-coords [y x])]
                     (get-in input coords)))}))

(defn low-point? [{:keys [point neighbours]}]
  (< point (reduce min neighbours)))

(defn part1 [input]
  (->> input
    (points-neighbours)
    (filter low-point?)
    (map :point)
    (map inc)
    (reduce +)))

(defn basins [input]
  (for [low-point (filter low-point? (points-neighbours input))]
    (loop [basin      #{(:coords low-point)}
           candidates (neighbour-coords (:coords low-point))]
      (let [new-coords (->> candidates
                         (filter #(not (nil? (get-in input %))))
                         (filter #(not= 9 (get-in input %))))]
        (if (empty? new-coords)
          basin
          (let [new-basin      (set/union basin (set new-coords))
                neighbours     (apply set/union (for [c new-coords] (set (neighbour-coords c))))
                new-candidates (set/difference neighbours new-basin)]
            (recur new-basin new-candidates)))))))

(defn part2 [input]
  (->> input
    (basins)
    (map count)
    (sort)
    (reverse)
    (take 3)
    (reduce *)))

(defn -main []
  (let [input (trace (parse "9.txt"))]
    (println "Part 1:" (part1 input))
    (println "Part 2:" (part2 input))))
