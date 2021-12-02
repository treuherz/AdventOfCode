(ns day2
  (:require [clojure.string :as str]))

(defn parse [file] (str/split-lines (slurp (clojure.java.io/resource file))))

(defn part1
  [input] (loop [[head & rest] input, pos 0, depth 0]
            (if (empty? head)
              (* pos depth)
              (let [[dir n-str] (str/split head #"\s")
                    n (Integer/parseInt n-str)]
                (condp = dir
                  "forward" (recur rest (+ pos n) depth)
                  "up" (recur rest pos (- depth n))
                  "down" (recur rest pos (+ depth n)))))))

(defn part2
  [input] (loop [[head & rest] input, pos 0, depth 0, aim 0]
            (if (empty? head)
              (* pos depth)
              (let [[dir n-str] (str/split head #"\s")
                    n (Integer/parseInt n-str)]
                (condp = dir
                  "forward" (recur rest (+ pos n) (+ depth (* aim n)) aim)
                  "up" (recur rest pos depth (- aim n))
                  "down" (recur rest pos depth (+ aim n)))))))

(defn -main
  [] (do (let [input (parse "2.txt")]
           (println "Part 1:" (part1 input))
           (println "Part 2:" (part2 input)))))
