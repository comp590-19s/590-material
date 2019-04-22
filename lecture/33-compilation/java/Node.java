public class Node {
    public int value;
    public Node next;

    public static Node cons(int value, Node next) {
        Node n = new Node();
        n.value = value;
        n.next = next;
        return n;
    }

    public static int first(Node node) {
        return node.value;
    }

    public static Node rest(Node node) {
        return node.next;
    }
}
