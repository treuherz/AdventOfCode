(ns day2
  (:require [clojure.string :as str]))

(defn parse
  [file] (->> file
           (clojure.java.io/resource)
           (slurp)
           (str/split-lines)
           (map #(str/split % #"\s" 2))
           (map (fn [[dir n-str]] {:dir dir, :n (Integer/parseInt n-str)}))))

(defn f1
  [{dir :dir, n :n}
   [pos, depth]]
  (condp = dir
    "forward" [(+ pos n) depth]
    "up" [pos (- depth n)]
    "down" [pos (+ depth n)]))

(defn f2
  [{dir :dir, n :n}
   [pos, depth, aim]]
  (condp = dir
    "forward" [(+ pos n) (+ depth (* aim n)) aim]
    "up" [pos depth (- aim n)]
    "down" [pos depth (+ aim n)]))

(defn run
  [input start move] (loop [[head & rest] input
                            coords start]
                       (if (nil? head)
                         (* (first coords) (second coords))
                         (recur rest (move head coords)))))

(defn -main
  [] (do (let [input (parse "2.txt")]
           (println "Part 1:" (run input [0 0] f1))
           (println "Part 2:" (run input [0 0 0] f2)))))
