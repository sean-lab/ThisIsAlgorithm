package ch08

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotNull
import kotlin.test.assertNull
import kotlin.test.assertTrue

class SimpleHashTableTest {
    @Test
    fun setThenGetReturnsValue() {
        val ht = SimpleHashTable(193)
        ht.set(418, 32114); ht.set(9, 514); ht.set(27, 8917); ht.set(1031, 286)
        assertEquals(32114, ht.get(418))
        assertEquals(514, ht.get(9))
        assertEquals(8917, ht.get(27))
        assertEquals(286, ht.get(1031))
    }

    @Test
    fun hashIsKeyModSize() {
        val ht = SimpleHashTable(193)
        assertEquals(418 % 193, ht.hash(418))
        assertEquals(9, ht.hash(9))
    }
}

class ChainingHashTableTest {
    private fun populate() = ChainingHashTable(12289).also { ht ->
        ht.set("MSFT", "Microsoft Corporation")
        ht.set("JAVA", "Sun Microsystems")
        ht.set("REDH", "Red Hat Linux")
        ht.set("APAC", "Apache Org")
        ht.set("ZYMZZ", "Unisys Ops Check")
        ht.set("IBM", "IBM Ltd.")
    }

    @Test
    fun getReturnsStoredValue() {
        val ht = populate()
        assertEquals("Microsoft Corporation", ht.get("MSFT"))
        assertEquals("Apache Org", ht.get("APAC"))
        assertEquals("IBM Ltd.", ht.get("IBM"))
    }

    @Test
    fun collisionKeysBothRetrievable() {
        val ht = populate()
        assertEquals(ChainingHashTable.hash("ZYMZZ", 12289), ChainingHashTable.hash("APAC", 12289))
        assertEquals("Unisys Ops Check", ht.get("ZYMZZ"))
        assertEquals("Apache Org", ht.get("APAC"))
    }

    @Test
    fun missingKeyReturnsNull() {
        val ht = populate()
        assertNull(ht.get("NOPE"))
    }
}

class OpenAddressHashTableTest {
    private fun populate() = OpenAddressHashTable(11).also { ht ->
        ht.set("MSFT", "Microsoft Corporation")
        ht.set("JAVA", "Sun Microsystems")
        ht.set("REDH", "Red Hat Linux")
        ht.set("APAC", "Apache Org")
        ht.set("ZYMZZ", "Unisys Ops Check")
        ht.set("IBM", "IBM Ltd.")
        ht.set("ORCL", "Oracle Corporation")
        ht.set("CSCO", "Cisco Systems, Inc.")
    }

    @Test
    fun getReturnsStoredValueAfterRehash() {
        val ht = populate()
        assertTrue(ht.tableSize > 11)
        assertEquals("Microsoft Corporation", ht.get("MSFT"))
        assertEquals("Apache Org", ht.get("APAC"))
        assertEquals("Cisco Systems, Inc.", ht.get("CSCO"))
    }

    @Test
    fun hash2IsInValidRange() {
        val size = 11
        val h = OpenAddressHashTable.hash2("APAC", size)
        assertTrue(h in 1..(size - 3 + 1))
    }
}
