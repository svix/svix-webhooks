package com.svix.test;

import static org.junit.Assert.*;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.svix.models.DashboardAccessOut;

import org.junit.Test;

public class ModelTests {
    @Test
    public void toJsonWorks() throws JsonProcessingException {
        DashboardAccessOut model = new DashboardAccessOut().token("asd");
        assertEquals("{\"token\":\"asd\"}", model.toJson());
    }

    @Test
    public void fromJsonWorks() throws JsonProcessingException {
        DashboardAccessOut expectedModel = new DashboardAccessOut().token("asd");
        DashboardAccessOut model = DashboardAccessOut.fromJson("{\"token\":\"asd\"}");
        assertEquals(expectedModel, model);
    }
}
