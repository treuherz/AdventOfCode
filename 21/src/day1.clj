(ns day1
  (:require [clojure.string :as str]))

(defn parse [file] (map #(Integer/parseInt %) (str/split-lines (slurp (clojure.java.io/resource file)))))

(defn part1
  [input] (->> input
            (partition 2 1)
            (filter #(let [[a b] %] (< a b)))
            (count)))


(defn part2
  [input] (->> input
            (partition 3 1)
            (map #(apply + %))
            (partition 2 1)
            (filter #(let [[a b] %] (< a b)))
            (count)))

(defn -main
  [] (do (let [input (parse "1.txt")]
       (println "Part 1:" (part1 input))
       (println "Part 2:" (part2 input)))))
